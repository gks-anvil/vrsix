from typing import Literal
from pydantic import BaseModel
from ga4gh.core.entity_models import Coding, Entity, IRI
from ga4gh.vrs.models import Allele


class RecordMetadata(BaseModel):

    recordIdentifier: str | None
    recordVersion: str | None
    derivedFrom: str | None
    dateRecordCreated: str | None
    contributions: list["Contribution"] | None


class InformationEntity(Entity):

    specifiedBy: "Method | IRI | None"
    contributions: list["Contribution"] | None
    reportedIn: list["Document | IRI"] | None
    dateAuthored: str | None
    derivedFrom: list["InformationEntity"] | None
    recordMetadata: RecordMetadata | None


class Document(Entity):

    specifiedBy: "Method | IRI | None"
    contributions: list["Contribution"] | None
    reportedIn: list["Document | IRI"] | None
    dateAuthored: str | None
    derivedFrom: list[InformationEntity] | None
    recordMetadata: RecordMetadata | None
    type: Literal["Document"] = "Document"
    subtype: Coding | None
    title: str | None
    urls: list[str] | None
    doi: str | None
    pmid: int | None


class Contribution(Entity):

    subtype: Coding | None
    date: str | None
    specifiedBy: "Method | IRI | None"
    contributions: list["Contribution"] | None
    reportedIn: list[Document | IRI] | None
    dateAuthored: str | None
    derivedFrom: list[InformationEntity] | None
    type: Literal["Method"] = "Method"
    subtype: Coding | None
    license: str | None


class Method(Entity):

    contributions: Contribution | None
    reportedIn: list[Document | IRI] | None
    dateAuthored: str | None
    derivedFrom: list[InformationEntity] | None
    recordMetadata: RecordMetadata | None
    type: Literal["Method"] = "Method"
    subtype: Coding | None
    license: str | None


class DataSet(Entity):

    specifiedBy: Method | IRI | None
    contributions: list[Contribution] | None
    reportedIn: list[Document | IRI] | None
    dateAuthored: str | None
    derivedFrom: list[InformationEntity] | None
    recordMetadata: RecordMetadata | None
    type: Literal["DataSet"] = "DataSet"
    subtype: Coding | None
    releaseDate: str | None
    version: str | None
    license: str | None


class Characteristic(BaseModel):

    name: str
    value: str
    valueOperator: bool | None


class StudyGroup(Entity):

    type: Literal["StudyGroup"] = "StudyGroup"
    memberCount: int | None
    isSubsetOf: "StudyGroup"
    characteristics: list[Characteristic] | None


class CohortAlleleFrequencyStudyResult(Entity):

    specifiedBy: Method | IRI | None
    contributions: list[Contribution] | None
    reportedIn: list[Document | IRI] | None
    dateAuthored: str | None
    recordMetadata: RecordMetadata | None
    ancillaryResults: dict | None
    qualityMeasures: dict | None
    type: Literal["CohortAlleleFrequencyStudyResult"] = "CohortAlleleFrequencyStudyResult"
    sourceDataSet = list[DataSet] | None
    focusAllele = Allele | str
    focusAlleleCount: int
    locusAlleleCount: int
    focusAlleleFrequency: float
    cohort: StudyGroup
    subCohortFrequency: list["CohortAlleleFrequencyStudyResult"] | None


RecordMetadata.model_rebuild()
Contribution.model_rebuild()
Document.model_rebuild()
InformationEntity.model_rebuild()
StudyGroup.model_rebuild()
CohortAlleleFrequencyStudyResult.model_rebuild()
