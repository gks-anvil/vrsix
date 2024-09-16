// TODO
// * double check that the is_empty function helps at all in light of ^
// * two parquet files, one for variants and one for patients
// * parallelism: https://github.com/zaeleus/noodles/issues/85
use noodles_vcf::{
    self as vcf,
    variant::record::{
        info::{self, field::Value as InfoValue},
        samples::{self, Sample},
    },
};
use std::path::PathBuf;
use std::time::Instant;
use polars::prelude::*;
use futures::{StreamExt, TryFutureExt, TryStreamExt};
use tokio::{fs::File as TkFile, io::BufReader};
use std::fs::File;

trait AnnotatedSample {
    fn get_allele_indices(
        &self,
        header: &vcf::Header,
    ) -> Result<Option<(usize, usize)>, Box<dyn std::error::Error>>;

    fn is_empty(&self) -> bool;
}

impl AnnotatedSample for vcf::record::samples::Sample<'_> {
    fn get_allele_indices(
        &self,
        header: &noodles_vcf::Header,
    ) -> Result<Option<(usize, usize)>, Box<dyn std::error::Error>> {
        if let samples::series::Value::Genotype(genotype) =
            self.get(&header, "GT").unwrap().unwrap().unwrap()
        {
            let mut gt_iter = genotype.iter();
            let first = gt_iter.next();
            let second = gt_iter.next();
            // // TODO check that iterator is exhausted here
            if let (Some(Ok((Some(a), _))), Some(Ok((Some(b), _)))) = (first, second) {
                return Ok(Some((a, b)));
            }
        }
        Ok(None)
    }

    fn is_empty(&self) -> bool {
        let src = self.as_ref();
        src == "./.:.:.:."
    }
}

// TODO move these to trait implementations/
// do this in a better way
fn unwrap_vrs_info_field(
    info_field: Option<Result<Option<InfoValue>, std::io::Error>>,
) -> Result<info::field::value::Array, Box<dyn std::error::Error>> {
    if let Some(Ok(Some(InfoValue::Array(array)))) = info_field {
        Ok(array)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Expected Array variant",
        )))
    }
}

fn vectorize_info_string_array(
    array: info::field::value::Array,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if let info::field::value::Array::String(array_elements) = array {
        let vec = array_elements
            .iter()
            .map(|cow_str| cow_str.unwrap().unwrap_or_default().to_string())
            .collect();
        return Ok(vec);
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Expected string Array variant",
        )))
    }
}
fn vectorize_info_int_array(
    array: info::field::value::Array,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    if let info::field::value::Array::String(array_elements) = array {
        let vec: Result<Vec<i32>, _> = array_elements
            .iter()
            .map(|cow_str| cow_str.unwrap().unwrap().parse::<i32>())
            .collect();
        return vec.map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Expected int Array variant",
        )))
    }
}

pub async fn to_parquet(vcf_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut reader = TkFile::open(vcf_path)
        .await
        .map(BufReader::new)
        .map(vcf::r#async::io::Reader::new)?;
    let header = reader.read_header().await?;
    let sample_names: Vec<&String> = header.sample_names().iter().collect();

    let mut patient_ids: Vec<&str> = vec![];
    let mut patient_vrs_ids: Vec<String> = vec![];

    let mut vrs_ids: Vec<String> = vec![];
    let mut vrs_chroms: Vec<String> = vec![];
    let mut vrs_starts: Vec<i32> = vec![];
    let mut vrs_ends: Vec<i32> = vec![];
    let mut vrs_states: Vec<String> = vec![];

    let mut records = reader.records();

    while let Some(record) = records.try_next().await? {
        let info = record.info();

        let mut record_vrs_ids = vectorize_info_string_array(unwrap_vrs_info_field(
            info.get(&header, "VRS_Allele_IDs"),
        )?)
        .unwrap();
        let mut record_vrs_starts =
            vectorize_info_int_array(unwrap_vrs_info_field(info.get(&header, "VRS_Starts"))?)
                .unwrap();
        let mut record_vrs_ends =
            vectorize_info_int_array(unwrap_vrs_info_field(info.get(&header, "VRS_Ends"))?)
                .unwrap();
        let mut record_vrs_states =
            vectorize_info_string_array(unwrap_vrs_info_field(info.get(&header, "VRS_States"))?)
                .unwrap();

        for (sample, pt_id) in record
            .samples()
            .iter()
            .zip(&sample_names)
            .filter(|(sample, _)| !sample.is_empty())
        {
            if let Ok(Some((a, b))) = sample.get_allele_indices(&header) {
                patient_ids.push(&pt_id);
                patient_ids.push(&pt_id);
                patient_vrs_ids.push(record_vrs_ids[a].clone());
                patient_vrs_ids.push(record_vrs_ids[b].clone());
            }
        }

        vrs_ids.append(&mut record_vrs_ids);
        let mut record_chroms = Vec::with_capacity(record_vrs_ids.len());
        record_chroms.resize(
            record_vrs_ids.len(),
            record.reference_sequence_name().to_string().clone(),
        );
        vrs_chroms.append(&mut record_chroms);
        vrs_starts.append(&mut record_vrs_starts);
        vrs_ends.append(&mut record_vrs_ends);
        vrs_states.append(&mut record_vrs_states);
    }

    let mut pts_df: DataFrame = df![
        "pt_id" => patient_ids,
        "vrs_id" => patient_vrs_ids
    ].unwrap();
    let pts_file = File::create("pts_vrs.parquet").expect("Couldn't open patients file to write");
    let _ = ParquetWriter::new(pts_file).finish(&mut pts_df);

    let mut vrs_df: DataFrame = df![
        "vrs_id" => vrs_ids,
        "chrom" => vrs_chroms,
        "start" => vrs_starts,
        "end" => vrs_ends,
        "state" => vrs_states,
    ].unwrap();
    let vrs_file = File::create("vrs.parquet").expect("Couldn't open variations file to write");
    let _ = ParquetWriter::new(vrs_file).finish(&mut vrs_df);

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
