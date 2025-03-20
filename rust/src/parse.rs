use log::error;

use crate::VcfError;
use noodles_vcf::{
    self as vcf,
    variant::record::info::{self, field::Value as InfoValue},
};
use pyo3::prelude::*;

pub enum VrsVcfField {
    VrsAlleleIds,
    VrsStarts,
    VrsEnds,
    VrsStates,
}

impl VrsVcfField {
    fn as_str(&self) -> &'static str {
        match self {
            VrsVcfField::VrsAlleleIds => "VRS_Allele_IDs",
            VrsVcfField::VrsStarts => "VRS_Starts",
            VrsVcfField::VrsEnds => "VRS_Ends",
            VrsVcfField::VrsStates => "VRS_States",
        }
    }
}

/// Retrieve data from INFO fields
/// Would prefer there was a way to do this in one pass instead of calling the method repeatedly
pub fn get_info_field(
    info: vcf::record::Info,
    header: &vcf::Header,
    field: VrsVcfField,
) -> Result<Vec<String>, PyErr> {
    if let Some(Ok(Some(InfoValue::Array(ids_array)))) = info.get(header, field.as_str()) {
        if let info::field::value::Array::String(array_elements) = ids_array {
            let vec = array_elements
                .iter()
                .map(|cow_str| cow_str.unwrap().unwrap_or_default().to_string())
                .collect();
            return Ok(vec);
        } else {
            error!("Unable to unpack `{:?}` as an array of values", ids_array);
            Err(VcfError::new_err("expected string array variant"))
        }
    } else {
        error!(
            "Unable to unpack {:?} from info fields: {:?}. Are annotations available?",
            field.as_str(),
            info
        );
        Err(VcfError::new_err("Expected Array variant"))
    }
}

pub fn cast_string_array_to_int(info_field_array: Vec<String>) -> Result<Vec<i32>, PyErr> {
    info_field_array
        .into_iter()
        .map(|s| {
            s.parse::<i32>()
                .map_err(|_e| VcfError::new_err("Unable to cast String instance to i32"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use noodles_vcf::{
        self as vcf,
        header::record::value::{map::Info, Map},
        header::FileFormat,
    };

    #[test]
    fn test_get_info_field() {
        pub const IDS_KEY: &str = "VRS_Allele_IDs";
        let ids_id = IDS_KEY;
        let ids_info = Map::<Info>::from(ids_id);
        pub const STARTS_KEY: &str = "VRS_Starts";
        let starts_id = STARTS_KEY;
        let starts_info = Map::<Info>::from(starts_id);
        pub const ENDS_KEY: &str = "VRS_Ends";
        let ends_id = ENDS_KEY;
        let ends_info = Map::<Info>::from(ends_id);
        pub const STATES_KEY: &str = "VRS_States";
        let states_id = STATES_KEY;
        let states_info = Map::<Info>::from(states_id);

        let header = vcf::Header::builder()
            .set_file_format(FileFormat::default())
            .add_info(ids_id, ids_info.clone())
            .add_info(starts_id, starts_info.clone())
            .add_info(ends_id, ends_info.clone())
            .add_info(states_id, states_info.clone())
            .build();
        let _infos = header.infos();
        // TODO finish this
    }
}
