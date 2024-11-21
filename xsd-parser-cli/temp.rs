// pub type Document = Document;
#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum AccountIdentification4ChoiceChoice {
    #[serde(rename = "IBAN")]
    Iban(Iban2007Identifier),
    Othr(GenericAccountIdentification1),
    __Unknown__(String),
}

impl Default for AccountIdentification4ChoiceChoice {
    fn default() -> AccountIdentification4ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AccountIdentification4ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct AccountIdentification4Choice {
    #[serde(flatten)]
    pub account_identification_4_choice_choice: AccountIdentification4ChoiceChoice,
}

impl Validate for AccountIdentification4Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum AccountSchemeName1ChoiceChoice {
    Cd(ExternalAccountIdentification1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for AccountSchemeName1ChoiceChoice {
    fn default() -> AccountSchemeName1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AccountSchemeName1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct AccountSchemeName1Choice {
    #[serde(flatten)]
    pub account_scheme_name_1_choice_choice: AccountSchemeName1ChoiceChoice,
}

impl Validate for AccountSchemeName1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType (pub xs::Decimal);

impl Validate for ActiveCurrencyAndAmountSimpleType {
    fn validate(&self) -> Result<(), String> { 
        if self.0 < "0".parse::<xs::Decimal>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 0.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
}

impl Validate for ActiveCurrencyAndAmount {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ActiveCurrencyCode (pub String);

impl Validate for ActiveCurrencyCode {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType (pub xs::Decimal);

impl Validate for ActiveOrHistoricCurrencyAndAmountSimpleType {
    fn validate(&self) -> Result<(), String> { 
        if self.0 < "0".parse::<xs::Decimal>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 0.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}

impl Validate for ActiveOrHistoricCurrencyAndAmount {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode (pub String);

impl Validate for ActiveOrHistoricCurrencyCode {}
#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
    __Unknown__(String),
}

impl Default for AddressType2Code {
    fn default() -> AddressType2Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AddressType2Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum AddressType3ChoiceChoice {
    Cd(AddressType2Code),
    Prtry(GenericIdentification30),
    __Unknown__(String),
}

impl Default for AddressType3ChoiceChoice {
    fn default() -> AddressType3ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AddressType3ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct AddressType3Choice {
    #[serde(flatten)]
    pub address_type_3_choice_choice: AddressType3ChoiceChoice,
}

impl Validate for AddressType3Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier (pub String);

impl Validate for AnyBICDec2014Identifier {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Bicfidec2014Identifier (pub String);

impl Validate for Bicfidec2014Identifier {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct BaseOneRate (pub xs::Decimal);

impl Validate for BaseOneRate {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct BatchBookingIndicator (pub bool);

impl Validate for BatchBookingIndicator {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct BranchAndFinancialInstitutionIdentification8 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification23,

    #[serde(rename = "BrnchId")]
    pub brnch_id: Option<BranchData5>,
}

impl Validate for BranchAndFinancialInstitutionIdentification8 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct BranchData5 {
    #[serde(rename = "Id")]
    pub id: Option<Max35Text>,

    #[serde(rename = "LEI")]
    pub lei: Option<Leiidentifier>,

    #[serde(rename = "Nm")]
    pub nm: Option<Max140Text>,

    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<PostalAddress27>,
}

impl Validate for BranchData5 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CashAccount40 {
    #[serde(rename = "Id")]
    pub id: Option<AccountIdentification4Choice>,

    #[serde(rename = "Tp")]
    pub tp: Option<CashAccountType2Choice>,

    #[serde(rename = "Ccy")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,

    #[serde(rename = "Nm")]
    pub nm: Option<Max70Text>,

    #[serde(rename = "Prxy")]
    pub prxy: Option<ProxyAccountIdentification1>,
}

impl Validate for CashAccount40 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum CashAccountType2ChoiceChoice {
    Cd(ExternalCashAccountType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for CashAccountType2ChoiceChoice {
    fn default() -> CashAccountType2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for CashAccountType2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CashAccountType2Choice {
    #[serde(flatten)]
    pub cash_account_type_2_choice_choice: CashAccountType2ChoiceChoice,
}

impl Validate for CashAccountType2Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum CategoryPurpose1ChoiceChoice {
    Cd(ExternalCategoryPurpose1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for CategoryPurpose1ChoiceChoice {
    fn default() -> CategoryPurpose1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for CategoryPurpose1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CategoryPurpose1Choice {
    #[serde(flatten)]
    pub category_purpose_1_choice_choice: CategoryPurpose1ChoiceChoice,
}

impl Validate for CategoryPurpose1Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ChargeBearerType1Code {
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "SHAR")]
    Shar,
    #[serde(rename = "SLEV")]
    Slev,
    __Unknown__(String),
}

impl Default for ChargeBearerType1Code {
    fn default() -> ChargeBearerType1Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ChargeBearerType1Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ChargeType3ChoiceChoice {
    Cd(ExternalChargeType1Code),
    Prtry(GenericIdentification3),
    __Unknown__(String),
}

impl Default for ChargeType3ChoiceChoice {
    fn default() -> ChargeType3ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ChargeType3ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ChargeType3Choice {
    #[serde(flatten)]
    pub charge_type_3_choice_choice: ChargeType3ChoiceChoice,
}

impl Validate for ChargeType3Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Charges16 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,

    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification8,

    #[serde(rename = "Tp")]
    pub tp: Option<ChargeType3Choice>,
}

impl Validate for Charges16 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ClearingChannel2Code {
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "RTNS")]
    Rtns,
    #[serde(rename = "MPNS")]
    Mpns,
    #[serde(rename = "BOOK")]
    Book,
    __Unknown__(String),
}

impl Default for ClearingChannel2Code {
    fn default() -> ClearingChannel2Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ClearingChannel2Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ClearingSystemIdentification2ChoiceChoice {
    Cd(ExternalClearingSystemIdentification1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for ClearingSystemIdentification2ChoiceChoice {
    fn default() -> ClearingSystemIdentification2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ClearingSystemIdentification2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ClearingSystemIdentification2Choice {
    #[serde(flatten)]
    pub clearing_system_identification_2_choice_choice: ClearingSystemIdentification2ChoiceChoice,
}

impl Validate for ClearingSystemIdentification2Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ClearingSystemIdentification3ChoiceChoice {
    Cd(ExternalCashClearingSystem1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for ClearingSystemIdentification3ChoiceChoice {
    fn default() -> ClearingSystemIdentification3ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ClearingSystemIdentification3ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ClearingSystemIdentification3Choice {
    #[serde(flatten)]
    pub clearing_system_identification_3_choice_choice: ClearingSystemIdentification3ChoiceChoice,
}

impl Validate for ClearingSystemIdentification3Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,

    #[serde(rename = "MmbId")]
    pub mmb_id: Max35Text,
}

impl Validate for ClearingSystemMemberIdentification2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Contact13 {
    #[serde(rename = "NmPrfx")]
    pub nm_prfx: Option<NamePrefix2Code>,

    #[serde(rename = "Nm")]
    pub nm: Option<Max140Text>,

    #[serde(rename = "PhneNb")]
    pub phne_nb: Option<PhoneNumber>,

    #[serde(rename = "MobNb")]
    pub mob_nb: Option<PhoneNumber>,

    #[serde(rename = "FaxNb")]
    pub fax_nb: Option<PhoneNumber>,

    #[serde(rename = "URLAdr")]
    pub url_adr: Option<Max2048Text>,

    #[serde(rename = "EmailAdr")]
    pub email_adr: Option<Max256Text>,

    #[serde(rename = "EmailPurp")]
    pub email_purp: Option<Max35Text>,

    #[serde(rename = "JobTitl")]
    pub job_titl: Option<Max35Text>,

    #[serde(rename = "Rspnsblty")]
    pub rspnsblty: Option<Max35Text>,

    #[serde(rename = "Dept")]
    pub dept: Option<Max70Text>,

    #[serde(rename = "Othr", default)]
    pub othr: Vec<OtherContact1>,

    #[serde(rename = "PrefrdMtd")]
    pub prefrd_mtd: Option<PreferredContactMethod2Code>,
}

impl Validate for Contact13 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct CountryCode (pub String);

impl Validate for CountryCode {}
#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
    __Unknown__(String),
}

impl Default for CreditDebitCode {
    fn default() -> CreditDebitCode {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for CreditDebitCode {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CreditTransferMandateData1 {
    #[serde(rename = "MndtId")]
    pub mndt_id: Option<Max35Text>,

    #[serde(rename = "Tp")]
    pub tp: Option<MandateTypeInformation2>,

    #[serde(rename = "DtOfSgntr")]
    pub dt_of_sgntr: Option<Isodate>,

    #[serde(rename = "DtOfVrfctn")]
    pub dt_of_vrfctn: Option<IsodateTime>,

    #[serde(rename = "ElctrncSgntr")]
    pub elctrnc_sgntr: Option<Max10KBinary>,

    #[serde(rename = "FrstPmtDt")]
    pub frst_pmt_dt: Option<Isodate>,

    #[serde(rename = "FnlPmtDt")]
    pub fnl_pmt_dt: Option<Isodate>,

    #[serde(rename = "Frqcy")]
    pub frqcy: Option<Frequency36Choice>,

    #[serde(rename = "Rsn")]
    pub rsn: Option<MandateSetupReason1Choice>,
}

impl Validate for CreditTransferMandateData1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CreditTransferTransaction64 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification13,

    #[serde(rename = "PmtTpInf")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,

    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: ActiveCurrencyAndAmount,

    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: Option<Isodate>,

    #[serde(rename = "SttlmPrty")]
    pub sttlm_prty: Option<Priority3Code>,

    #[serde(rename = "SttlmTmIndctn")]
    pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,

    #[serde(rename = "SttlmTmReq")]
    pub sttlm_tm_req: Option<SettlementTimeRequest2>,

    #[serde(rename = "AccptncDtTm")]
    pub accptnc_dt_tm: Option<IsodateTime>,

    #[serde(rename = "PoolgAdjstmntDt")]
    pub poolg_adjstmnt_dt: Option<Isodate>,

    #[serde(rename = "InstdAmt")]
    pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "XchgRate")]
    pub xchg_rate: Option<BaseOneRate>,

    #[serde(rename = "ChrgBr")]
    pub chrg_br: ChargeBearerType1Code,

    #[serde(rename = "ChrgsInf", default)]
    pub chrgs_inf: Vec<Charges16>,

    #[serde(rename = "MndtRltdInf")]
    pub mndt_rltd_inf: Option<CreditTransferMandateData1>,

    #[serde(rename = "PrvsInstgAgt1")]
    pub prvs_instg_agt_1: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "PrvsInstgAgt1Acct")]
    pub prvs_instg_agt_1_acct: Option<CashAccount40>,

    #[serde(rename = "PrvsInstgAgt2")]
    pub prvs_instg_agt_2: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "PrvsInstgAgt2Acct")]
    pub prvs_instg_agt_2_acct: Option<CashAccount40>,

    #[serde(rename = "PrvsInstgAgt3")]
    pub prvs_instg_agt_3: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "PrvsInstgAgt3Acct")]
    pub prvs_instg_agt_3_acct: Option<CashAccount40>,

    #[serde(rename = "InstgAgt")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "InstdAgt")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "IntrmyAgt1")]
    pub intrmy_agt_1: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "IntrmyAgt1Acct")]
    pub intrmy_agt_1_acct: Option<CashAccount40>,

    #[serde(rename = "IntrmyAgt2")]
    pub intrmy_agt_2: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "IntrmyAgt2Acct")]
    pub intrmy_agt_2_acct: Option<CashAccount40>,

    #[serde(rename = "IntrmyAgt3")]
    pub intrmy_agt_3: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "IntrmyAgt3Acct")]
    pub intrmy_agt_3_acct: Option<CashAccount40>,

    #[serde(rename = "UltmtDbtr")]
    pub ultmt_dbtr: Option<PartyIdentification272>,

    #[serde(rename = "InitgPty")]
    pub initg_pty: Option<PartyIdentification272>,

    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification272,

    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: Option<CashAccount40>,

    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,

    #[serde(rename = "DbtrAgtAcct")]
    pub dbtr_agt_acct: Option<CashAccount40>,

    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,

    #[serde(rename = "CdtrAgtAcct")]
    pub cdtr_agt_acct: Option<CashAccount40>,

    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification272,

    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: Option<CashAccount40>,

    #[serde(rename = "UltmtCdtr")]
    pub ultmt_cdtr: Option<PartyIdentification272>,

    #[serde(rename = "InstrForCdtrAgt", default)]
    pub instr_for_cdtr_agt: Vec<InstructionForCreditorAgent3>,

    #[serde(rename = "InstrForNxtAgt", default)]
    pub instr_for_nxt_agt: Vec<InstructionForNextAgent1>,

    #[serde(rename = "Purp")]
    pub purp: Option<Purpose2Choice>,

    #[serde(rename = "RgltryRptg", default)]
    pub rgltry_rptg: Vec<RegulatoryReporting3>,

    #[serde(rename = "Tax")]
    pub tax: Option<TaxData1>,

    #[serde(rename = "RltdRmtInf", default)]
    pub rltd_rmt_inf: Vec<RemittanceLocation8>,

    #[serde(rename = "RmtInf")]
    pub rmt_inf: Option<RemittanceInformation22>,

    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1>,
}

impl Validate for CreditTransferTransaction64 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CreditorReferenceInformation3 {
    #[serde(rename = "Tp")]
    pub tp: Option<CreditorReferenceType3>,

    #[serde(rename = "Ref")]
    pub _ref: Option<Max35Text>,
}

impl Validate for CreditorReferenceInformation3 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum CreditorReferenceType2ChoiceChoice {
    Cd(ExternalCreditorReferenceType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for CreditorReferenceType2ChoiceChoice {
    fn default() -> CreditorReferenceType2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for CreditorReferenceType2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CreditorReferenceType2Choice {
    #[serde(flatten)]
    pub creditor_reference_type_2_choice_choice: CreditorReferenceType2ChoiceChoice,
}

impl Validate for CreditorReferenceType2Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct CreditorReferenceType3 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: CreditorReferenceType2Choice,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for CreditorReferenceType3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DateAndPlaceOfBirth1 {
    #[serde(rename = "BirthDt")]
    pub birth_dt: Isodate,

    #[serde(rename = "PrvcOfBirth")]
    pub prvc_of_birth: Option<Max35Text>,

    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: Max35Text,

    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: CountryCode,
}

impl Validate for DateAndPlaceOfBirth1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DateAndType1 {
    #[serde(rename = "Tp")]
    pub tp: DateType2Choice,

    #[serde(rename = "Dt")]
    pub dt: Isodate,
}

impl Validate for DateAndType1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DatePeriod2 {
    #[serde(rename = "FrDt")]
    pub fr_dt: Isodate,

    #[serde(rename = "ToDt")]
    pub to_dt: Isodate,
}

impl Validate for DatePeriod2 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum DateType2ChoiceChoice {
    Cd(ExternalDateType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for DateType2ChoiceChoice {
    fn default() -> DateType2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DateType2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DateType2Choice {
    #[serde(flatten)]
    pub date_type_2_choice_choice: DateType2ChoiceChoice,
}

impl Validate for DateType2Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct DecimalNumber (pub xs::Decimal);

impl Validate for DecimalNumber {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Document {
    #[serde(rename = "FIToFICstmrCdtTrf")]
    pub fi_to_fi_cstmr_cdt_trf: FitoFICustomerCreditTransferV12,
}

impl Validate for Document {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentAdjustment1 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,

    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,

    #[serde(rename = "Rsn")]
    pub rsn: Option<Max4Text>,

    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Option<Max140Text>,
}

impl Validate for DocumentAdjustment1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentAmount1 {
    #[serde(rename = "Tp")]
    pub tp: DocumentAmountType1Choice,

    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl Validate for DocumentAmount1 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum DocumentAmountType1ChoiceChoice {
    Cd(ExternalDocumentAmountType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for DocumentAmountType1ChoiceChoice {
    fn default() -> DocumentAmountType1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DocumentAmountType1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentAmountType1Choice {
    #[serde(flatten)]
    pub document_amount_type_1_choice_choice: DocumentAmountType1ChoiceChoice,
}

impl Validate for DocumentAmountType1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp")]
    pub tp: Option<DocumentLineType1>,

    #[serde(rename = "Nb")]
    pub nb: Option<Max35Text>,

    #[serde(rename = "RltdDt")]
    pub rltd_dt: Option<Isodate>,
}

impl Validate for DocumentLineIdentification1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentLineInformation2 {
    #[serde(rename = "Id", default)]
    pub id: Vec<DocumentLineIdentification1>,

    #[serde(rename = "Desc")]
    pub desc: Option<Max2048Text>,

    #[serde(rename = "Amt")]
    pub amt: Option<RemittanceAmount4>,
}

impl Validate for DocumentLineInformation2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentLineType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: DocumentLineType1Choice,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for DocumentLineType1 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum DocumentLineType1ChoiceChoice {
    Cd(ExternalDocumentLineType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for DocumentLineType1ChoiceChoice {
    fn default() -> DocumentLineType1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DocumentLineType1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentLineType1Choice {
    #[serde(flatten)]
    pub document_line_type_1_choice_choice: DocumentLineType1ChoiceChoice,
}

impl Validate for DocumentLineType1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: DocumentType2Choice,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for DocumentType1 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum DocumentType2ChoiceChoice {
    Cd(ExternalDocumentType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for DocumentType2ChoiceChoice {
    fn default() -> DocumentType2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DocumentType2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct DocumentType2Choice {
    #[serde(flatten)]
    pub document_type_2_choice_choice: DocumentType2ChoiceChoice,
}

impl Validate for DocumentType2Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Exact2NumericText (pub String);

impl Validate for Exact2NumericText {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText (pub String);

impl Validate for Exact4AlphaNumericText {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code (pub String);

impl Validate for ExternalAccountIdentification1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code (pub String);

impl Validate for ExternalCashAccountType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalCashClearingSystem1Code (pub String);

impl Validate for ExternalCashClearingSystem1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 3 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 3 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalCategoryPurpose1Code (pub String);

impl Validate for ExternalCategoryPurpose1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalChargeType1Code (pub String);

impl Validate for ExternalChargeType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code (pub String);

impl Validate for ExternalClearingSystemIdentification1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 5 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 5 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalCreditorAgentInstruction1Code (pub String);

impl Validate for ExternalCreditorAgentInstruction1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalCreditorReferenceType1Code (pub String);

impl Validate for ExternalCreditorReferenceType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalDateType1Code (pub String);

impl Validate for ExternalDateType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalDocumentAmountType1Code (pub String);

impl Validate for ExternalDocumentAmountType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalDocumentLineType1Code (pub String);

impl Validate for ExternalDocumentLineType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code (pub String);

impl Validate for ExternalDocumentType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code (pub String);

impl Validate for ExternalFinancialInstitutionIdentification1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalGarnishmentType1Code (pub String);

impl Validate for ExternalGarnishmentType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalLocalInstrument1Code (pub String);

impl Validate for ExternalLocalInstrument1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 35 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 35 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalMandateSetupReason1Code (pub String);

impl Validate for ExternalMandateSetupReason1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code (pub String);

impl Validate for ExternalOrganisationIdentification1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code (pub String);

impl Validate for ExternalPersonIdentification1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code (pub String);

impl Validate for ExternalProxyAccountType1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalPurpose1Code (pub String);

impl Validate for ExternalPurpose1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalServiceLevel1Code (pub String);

impl Validate for ExternalServiceLevel1Code {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct FitoFICustomerCreditTransferV12 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader113,

    #[serde(rename = "CdtTrfTxInf", default)]
    pub cdt_trf_tx_inf: Vec<CreditTransferTransaction64>,

    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1>,
}

impl Validate for FitoFICustomerCreditTransferV12 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum FinancialIdentificationSchemeName1ChoiceChoice {
    Cd(ExternalFinancialInstitutionIdentification1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for FinancialIdentificationSchemeName1ChoiceChoice {
    fn default() -> FinancialIdentificationSchemeName1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FinancialIdentificationSchemeName1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub financial_identification_scheme_name_1_choice_choice: FinancialIdentificationSchemeName1ChoiceChoice,
}

impl Validate for FinancialIdentificationSchemeName1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct FinancialInstitutionIdentification23 {
    #[serde(rename = "BICFI")]
    pub bicfi: Option<Bicfidec2014Identifier>,

    #[serde(rename = "ClrSysMmbId")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,

    #[serde(rename = "LEI")]
    pub lei: Option<Leiidentifier>,

    #[serde(rename = "Nm")]
    pub nm: Option<Max140Text>,

    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<PostalAddress27>,

    #[serde(rename = "Othr")]
    pub othr: Option<GenericFinancialIdentification1>,
}

impl Validate for FinancialInstitutionIdentification23 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Frequency36ChoiceChoice {
    Tp(Frequency6Code),
    Prd(FrequencyPeriod1),
    PtInTm(FrequencyAndMoment1),
    __Unknown__(String),
}

impl Default for Frequency36ChoiceChoice {
    fn default() -> Frequency36ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Frequency36ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Frequency36Choice {
    #[serde(flatten)]
    pub frequency_36_choice_choice: Frequency36ChoiceChoice,
}

impl Validate for Frequency36Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Frequency6Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "FRTN")]
    Frtn,
    __Unknown__(String),
}

impl Default for Frequency6Code {
    fn default() -> Frequency6Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Frequency6Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct FrequencyAndMoment1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,

    #[serde(rename = "PtInTm")]
    pub pt_in_tm: Exact2NumericText,
}

impl Validate for FrequencyAndMoment1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct FrequencyPeriod1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,

    #[serde(rename = "CntPerPrd")]
    pub cnt_per_prd: DecimalNumber,
}

impl Validate for FrequencyPeriod1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Garnishment4 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType1,

    #[serde(rename = "Grnshee")]
    pub grnshee: Option<PartyIdentification272>,

    #[serde(rename = "GrnshmtAdmstr")]
    pub grnshmt_admstr: Option<PartyIdentification272>,

    #[serde(rename = "RefNb")]
    pub ref_nb: Option<Max140Text>,

    #[serde(rename = "Dt")]
    pub dt: Option<Isodate>,

    #[serde(rename = "RmtdAmt")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "FmlyMdclInsrncInd")]
    pub fmly_mdcl_insrnc_ind: Option<TrueFalseIndicator>,

    #[serde(rename = "MplyeeTermntnInd")]
    pub mplyee_termntn_ind: Option<TrueFalseIndicator>,
}

impl Validate for Garnishment4 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GarnishmentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: GarnishmentType1Choice,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for GarnishmentType1 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum GarnishmentType1ChoiceChoice {
    Cd(ExternalGarnishmentType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for GarnishmentType1ChoiceChoice {
    fn default() -> GarnishmentType1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for GarnishmentType1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GarnishmentType1Choice {
    #[serde(flatten)]
    pub garnishment_type_1_choice_choice: GarnishmentType1ChoiceChoice,
}

impl Validate for GarnishmentType1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GenericAccountIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max34Text,

    #[serde(rename = "SchmeNm")]
    pub schme_nm: Option<AccountSchemeName1Choice>,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for GenericAccountIdentification1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GenericFinancialIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max35Text,

    #[serde(rename = "SchmeNm")]
    pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for GenericFinancialIdentification1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GenericIdentification3 {
    #[serde(rename = "Id")]
    pub id: Max35Text,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for GenericIdentification3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,

    #[serde(rename = "Issr")]
    pub issr: Max35Text,

    #[serde(rename = "SchmeNm")]
    pub schme_nm: Option<Max35Text>,
}

impl Validate for GenericIdentification30 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GenericOrganisationIdentification3 {
    #[serde(rename = "Id")]
    pub id: Max256Text,

    #[serde(rename = "SchmeNm")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for GenericOrganisationIdentification3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GenericPersonIdentification2 {
    #[serde(rename = "Id")]
    pub id: Max256Text,

    #[serde(rename = "SchmeNm")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,

    #[serde(rename = "Issr")]
    pub issr: Option<Max35Text>,
}

impl Validate for GenericPersonIdentification2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct GroupHeader113 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,

    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsodateTime,

    #[serde(rename = "BtchBookg")]
    pub btch_bookg: Option<BatchBookingIndicator>,

    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericText,

    #[serde(rename = "CtrlSum")]
    pub ctrl_sum: Option<DecimalNumber>,

    #[serde(rename = "TtlIntrBkSttlmAmt")]
    pub ttl_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,

    #[serde(rename = "IntrBkSttlmDt")]
    pub intr_bk_sttlm_dt: Option<Isodate>,

    #[serde(rename = "SttlmInf")]
    pub sttlm_inf: SettlementInstruction15,

    #[serde(rename = "PmtTpInf")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,

    #[serde(rename = "InstgAgt")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "InstdAgt")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl Validate for GroupHeader113 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Iban2007Identifier (pub String);

impl Validate for Iban2007Identifier {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Isodate (pub xs::Date);

impl Validate for Isodate {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct IsodateTime (pub xs::DateTime);

impl Validate for IsodateTime {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Isotime (pub xs::Time);

impl Validate for Isotime {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Isoyear (pub xs::GYear);

impl Validate for Isoyear {}
#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Instruction4Code {
    #[serde(rename = "PHOA")]
    Phoa,
    #[serde(rename = "TELA")]
    Tela,
    __Unknown__(String),
}

impl Default for Instruction4Code {
    fn default() -> Instruction4Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Instruction4Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct InstructionForCreditorAgent3 {
    #[serde(rename = "Cd")]
    pub cd: Option<ExternalCreditorAgentInstruction1Code>,

    #[serde(rename = "InstrInf")]
    pub instr_inf: Option<Max140Text>,
}

impl Validate for InstructionForCreditorAgent3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct InstructionForNextAgent1 {
    #[serde(rename = "Cd")]
    pub cd: Option<Instruction4Code>,

    #[serde(rename = "InstrInf")]
    pub instr_inf: Option<Max140Text>,
}

impl Validate for InstructionForNextAgent1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Leiidentifier (pub String);

impl Validate for Leiidentifier {}
#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum LocalInstrument2ChoiceChoice {
    Cd(ExternalLocalInstrument1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for LocalInstrument2ChoiceChoice {
    fn default() -> LocalInstrument2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for LocalInstrument2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct LocalInstrument2Choice {
    #[serde(flatten)]
    pub local_instrument_2_choice_choice: LocalInstrument2ChoiceChoice,
}

impl Validate for LocalInstrument2Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum MandateClassification1ChoiceChoice {
    Cd(MandateClassification1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for MandateClassification1ChoiceChoice {
    fn default() -> MandateClassification1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MandateClassification1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct MandateClassification1Choice {
    #[serde(flatten)]
    pub mandate_classification_1_choice_choice: MandateClassification1ChoiceChoice,
}

impl Validate for MandateClassification1Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum MandateClassification1Code {
    #[serde(rename = "FIXE")]
    Fixe,
    #[serde(rename = "USGB")]
    Usgb,
    #[serde(rename = "VARI")]
    Vari,
    __Unknown__(String),
}

impl Default for MandateClassification1Code {
    fn default() -> MandateClassification1Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MandateClassification1Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum MandateSetupReason1ChoiceChoice {
    Cd(ExternalMandateSetupReason1Code),
    Prtry(Max70Text),
    __Unknown__(String),
}

impl Default for MandateSetupReason1ChoiceChoice {
    fn default() -> MandateSetupReason1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MandateSetupReason1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct MandateSetupReason1Choice {
    #[serde(flatten)]
    pub mandate_setup_reason_1_choice_choice: MandateSetupReason1ChoiceChoice,
}

impl Validate for MandateSetupReason1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct MandateTypeInformation2 {
    #[serde(rename = "SvcLvl")]
    pub svc_lvl: Option<ServiceLevel8Choice>,

    #[serde(rename = "LclInstrm")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,

    #[serde(rename = "CtgyPurp")]
    pub ctgy_purp: Option<CategoryPurpose1Choice>,

    #[serde(rename = "Clssfctn")]
    pub clssfctn: Option<MandateClassification1Choice>,
}

impl Validate for MandateTypeInformation2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max10KBinary (pub String);

impl Validate for Max10KBinary {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 10240 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 10240 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max10Text (pub String);

impl Validate for Max10Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 10 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 10 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max128Text (pub String);

impl Validate for Max128Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 128 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 128 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max140Text (pub String);

impl Validate for Max140Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 140 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 140 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max15NumericText (pub String);

impl Validate for Max15NumericText {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max16Text (pub String);

impl Validate for Max16Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 16 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 16 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max2048Text (pub String);

impl Validate for Max2048Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 2048 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 2048 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max256Text (pub String);

impl Validate for Max256Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 256 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 256 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max34Text (pub String);

impl Validate for Max34Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 34 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 34 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max350Text (pub String);

impl Validate for Max350Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 350 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 350 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max35Text (pub String);

impl Validate for Max35Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 35 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 35 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max4Text (pub String);

impl Validate for Max4Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 4 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Max70Text (pub String);

impl Validate for Max70Text {
    fn validate(&self) -> Result<(), String> { 
        if self.0.len() < 1 {
            return Err(format!("MinLength validation error. \nExpected: 0 length >= 1 \nActual: 0 length == {}", self.0.len()));
        }
        if self.0.len() > 70 {
            return Err(format!("MaxLength validation error. \nExpected: 0 length <= 70 \nActual: 0 length == {}", self.0.len()));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct NameAndAddress18 {
    #[serde(rename = "Nm")]
    pub nm: Max140Text,

    #[serde(rename = "Adr")]
    pub adr: PostalAddress27,
}

impl Validate for NameAndAddress18 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
    __Unknown__(String),
}

impl Default for NamePrefix2Code {
    fn default() -> NamePrefix2Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for NamePrefix2Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Number (pub xs::Decimal);

impl Validate for Number {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct OrganisationIdentification39 {
    #[serde(rename = "AnyBIC")]
    pub any_bic: Option<AnyBICDec2014Identifier>,

    #[serde(rename = "LEI")]
    pub lei: Option<Leiidentifier>,

    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericOrganisationIdentification3>,
}

impl Validate for OrganisationIdentification39 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum OrganisationIdentificationSchemeName1ChoiceChoice {
    Cd(ExternalOrganisationIdentification1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for OrganisationIdentificationSchemeName1ChoiceChoice {
    fn default() -> OrganisationIdentificationSchemeName1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for OrganisationIdentificationSchemeName1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub organisation_identification_scheme_name_1_choice_choice: OrganisationIdentificationSchemeName1ChoiceChoice,
}

impl Validate for OrganisationIdentificationSchemeName1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    pub chanl_tp: Max4Text,

    #[serde(rename = "Id")]
    pub id: Option<Max128Text>,
}

impl Validate for OtherContact1 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Party52ChoiceChoice {
    OrgId(OrganisationIdentification39),
    PrvtId(PersonIdentification18),
    __Unknown__(String),
}

impl Default for Party52ChoiceChoice {
    fn default() -> Party52ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Party52ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Party52Choice {
    #[serde(flatten)]
    pub party_52_choice_choice: Party52ChoiceChoice,
}

impl Validate for Party52Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct PartyIdentification272 {
    #[serde(rename = "Nm")]
    pub nm: Option<Max140Text>,

    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<PostalAddress27>,

    #[serde(rename = "Id")]
    pub id: Option<Party52Choice>,

    #[serde(rename = "CtryOfRes")]
    pub ctry_of_res: Option<CountryCode>,

    #[serde(rename = "CtctDtls")]
    pub ctct_dtls: Option<Contact13>,
}

impl Validate for PartyIdentification272 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct PaymentIdentification13 {
    #[serde(rename = "InstrId")]
    pub instr_id: Option<Max35Text>,

    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: Max35Text,

    #[serde(rename = "TxId")]
    pub tx_id: Option<Max35Text>,

    #[serde(rename = "UETR")]
    pub uetr: Option<Uuidv4Identifier>,

    #[serde(rename = "ClrSysRef")]
    pub clr_sys_ref: Option<Max35Text>,
}

impl Validate for PaymentIdentification13 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct PaymentTypeInformation28 {
    #[serde(rename = "InstrPrty")]
    pub instr_prty: Option<Priority2Code>,

    #[serde(rename = "ClrChanl")]
    pub clr_chanl: Option<ClearingChannel2Code>,

    #[serde(rename = "SvcLvl", default)]
    pub svc_lvl: Vec<ServiceLevel8Choice>,

    #[serde(rename = "LclInstrm")]
    pub lcl_instrm: Option<LocalInstrument2Choice>,

    #[serde(rename = "CtgyPurp")]
    pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl Validate for PaymentTypeInformation28 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct PercentageRate (pub xs::Decimal);

impl Validate for PercentageRate {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct PersonIdentification18 {
    #[serde(rename = "DtAndPlcOfBirth")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,

    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericPersonIdentification2>,
}

impl Validate for PersonIdentification18 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum PersonIdentificationSchemeName1ChoiceChoice {
    Cd(ExternalPersonIdentification1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for PersonIdentificationSchemeName1ChoiceChoice {
    fn default() -> PersonIdentificationSchemeName1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for PersonIdentificationSchemeName1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct PersonIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub person_identification_scheme_name_1_choice_choice: PersonIdentificationSchemeName1ChoiceChoice,
}

impl Validate for PersonIdentificationSchemeName1Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct PhoneNumber (pub String);

impl Validate for PhoneNumber {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct PostalAddress27 {
    #[serde(rename = "AdrTp")]
    pub adr_tp: Option<AddressType3Choice>,

    #[serde(rename = "CareOf")]
    pub care_of: Option<Max140Text>,

    #[serde(rename = "Dept")]
    pub dept: Option<Max70Text>,

    #[serde(rename = "SubDept")]
    pub sub_dept: Option<Max70Text>,

    #[serde(rename = "StrtNm")]
    pub strt_nm: Option<Max140Text>,

    #[serde(rename = "BldgNb")]
    pub bldg_nb: Option<Max16Text>,

    #[serde(rename = "BldgNm")]
    pub bldg_nm: Option<Max140Text>,

    #[serde(rename = "Flr")]
    pub flr: Option<Max70Text>,

    #[serde(rename = "UnitNb")]
    pub unit_nb: Option<Max16Text>,

    #[serde(rename = "PstBx")]
    pub pst_bx: Option<Max16Text>,

    #[serde(rename = "Room")]
    pub room: Option<Max70Text>,

    #[serde(rename = "PstCd")]
    pub pst_cd: Option<Max16Text>,

    #[serde(rename = "TwnNm")]
    pub twn_nm: Option<Max140Text>,

    #[serde(rename = "TwnLctnNm")]
    pub twn_lctn_nm: Option<Max140Text>,

    #[serde(rename = "DstrctNm")]
    pub dstrct_nm: Option<Max140Text>,

    #[serde(rename = "CtrySubDvsn")]
    pub ctry_sub_dvsn: Option<Max35Text>,

    #[serde(rename = "Ctry")]
    pub ctry: Option<CountryCode>,

    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
}

impl Validate for PostalAddress27 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum PreferredContactMethod2Code {
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "CELL")]
    Cell,
    #[serde(rename = "ONLI")]
    Onli,
    #[serde(rename = "PHON")]
    Phon,
    __Unknown__(String),
}

impl Default for PreferredContactMethod2Code {
    fn default() -> PreferredContactMethod2Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for PreferredContactMethod2Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Priority2Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    __Unknown__(String),
}

impl Default for Priority2Code {
    fn default() -> Priority2Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Priority2Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Priority3Code {
    #[serde(rename = "URGT")]
    Urgt,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    __Unknown__(String),
}

impl Default for Priority3Code {
    fn default() -> Priority3Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Priority3Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp")]
    pub tp: Option<ProxyAccountType1Choice>,

    #[serde(rename = "Id")]
    pub id: Max2048Text,
}

impl Validate for ProxyAccountIdentification1 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ProxyAccountType1ChoiceChoice {
    Cd(ExternalProxyAccountType1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for ProxyAccountType1ChoiceChoice {
    fn default() -> ProxyAccountType1ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ProxyAccountType1ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ProxyAccountType1Choice {
    #[serde(flatten)]
    pub proxy_account_type_1_choice_choice: ProxyAccountType1ChoiceChoice,
}

impl Validate for ProxyAccountType1Choice {}




#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum Purpose2ChoiceChoice {
    Cd(ExternalPurpose1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for Purpose2ChoiceChoice {
    fn default() -> Purpose2ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Purpose2ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct Purpose2Choice {
    #[serde(flatten)]
    pub purpose_2_choice_choice: Purpose2ChoiceChoice,
}

impl Validate for Purpose2Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ReferredDocumentInformation8 {
    #[serde(rename = "Tp")]
    pub tp: Option<DocumentType1>,

    #[serde(rename = "Nb")]
    pub nb: Option<Max35Text>,

    #[serde(rename = "RltdDt")]
    pub rltd_dt: Option<DateAndType1>,

    #[serde(rename = "LineDtls", default)]
    pub line_dtls: Vec<DocumentLineInformation2>,
}

impl Validate for ReferredDocumentInformation8 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm")]
    pub nm: Option<Max140Text>,

    #[serde(rename = "Ctry")]
    pub ctry: Option<CountryCode>,
}

impl Validate for RegulatoryAuthority2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd")]
    pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,

    #[serde(rename = "Authrty")]
    pub authrty: Option<RegulatoryAuthority2>,

    #[serde(rename = "Dtls", default)]
    pub dtls: Vec<StructuredRegulatoryReporting3>,
}

impl Validate for RegulatoryReporting3 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum RegulatoryReportingType1Code {
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "BOTH")]
    Both,
    __Unknown__(String),
}

impl Default for RegulatoryReportingType1Code {
    fn default() -> RegulatoryReportingType1Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for RegulatoryReportingType1Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct RemittanceAmount4 {
    #[serde(rename = "RmtAmtAndTp", default)]
    pub rmt_amt_and_tp: Vec<DocumentAmount1>,

    #[serde(rename = "AdjstmntAmtAndRsn", default)]
    pub adjstmnt_amt_and_rsn: Vec<DocumentAdjustment1>,
}

impl Validate for RemittanceAmount4 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct RemittanceInformation22 {
    #[serde(rename = "Ustrd", default)]
    pub ustrd: Vec<Max140Text>,

    #[serde(rename = "Strd", default)]
    pub strd: Vec<StructuredRemittanceInformation18>,
}

impl Validate for RemittanceInformation22 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct RemittanceLocation8 {
    #[serde(rename = "RmtId")]
    pub rmt_id: Option<Max35Text>,

    #[serde(rename = "RmtLctnDtls", default)]
    pub rmt_lctn_dtls: Vec<RemittanceLocationData2>,
}

impl Validate for RemittanceLocation8 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct RemittanceLocationData2 {
    #[serde(rename = "Mtd")]
    pub mtd: RemittanceLocationMethod2Code,

    #[serde(rename = "ElctrncAdr")]
    pub elctrnc_adr: Option<Max2048Text>,

    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<NameAndAddress18>,
}

impl Validate for RemittanceLocationData2 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum RemittanceLocationMethod2Code {
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "EDIC")]
    Edic,
    #[serde(rename = "URID")]
    Urid,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "SMSM")]
    Smsm,
    __Unknown__(String),
}

impl Default for RemittanceLocationMethod2Code {
    fn default() -> RemittanceLocationMethod2Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for RemittanceLocationMethod2Code {}



#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ServiceLevel8ChoiceChoice {
    Cd(ExternalServiceLevel1Code),
    Prtry(Max35Text),
    __Unknown__(String),
}

impl Default for ServiceLevel8ChoiceChoice {
    fn default() -> ServiceLevel8ChoiceChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ServiceLevel8ChoiceChoice {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct ServiceLevel8Choice {
    #[serde(flatten)]
    pub service_level_8_choice_choice: ServiceLevel8ChoiceChoice,
}

impl Validate for ServiceLevel8Choice {}




#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct SettlementDateTimeIndication1 {
    #[serde(rename = "DbtDtTm")]
    pub dbt_dt_tm: Option<IsodateTime>,

    #[serde(rename = "CdtDtTm")]
    pub cdt_dt_tm: Option<IsodateTime>,
}

impl Validate for SettlementDateTimeIndication1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct SettlementInstruction15 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: SettlementMethod1Code,

    #[serde(rename = "SttlmAcct")]
    pub sttlm_acct: Option<CashAccount40>,

    #[serde(rename = "ClrSys")]
    pub clr_sys: Option<ClearingSystemIdentification3Choice>,

    #[serde(rename = "InstgRmbrsmntAgt")]
    pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "InstgRmbrsmntAgtAcct")]
    pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,

    #[serde(rename = "InstdRmbrsmntAgt")]
    pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "InstdRmbrsmntAgtAcct")]
    pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,

    #[serde(rename = "ThrdRmbrsmntAgt")]
    pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,

    #[serde(rename = "ThrdRmbrsmntAgtAcct")]
    pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}

impl Validate for SettlementInstruction15 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum SettlementMethod1Code {
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "INGA")]
    Inga,
    #[serde(rename = "COVE")]
    Cove,
    #[serde(rename = "CLRG")]
    Clrg,
    __Unknown__(String),
}

impl Default for SettlementMethod1Code {
    fn default() -> SettlementMethod1Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SettlementMethod1Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct SettlementTimeRequest2 {
    #[serde(rename = "CLSTm")]
    pub cls_tm: Option<Isotime>,

    #[serde(rename = "TillTm")]
    pub till_tm: Option<Isotime>,

    #[serde(rename = "FrTm")]
    pub fr_tm: Option<Isotime>,

    #[serde(rename = "RjctTm")]
    pub rjct_tm: Option<Isotime>,
}

impl Validate for SettlementTimeRequest2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct StructuredRegulatoryReporting3 {
    #[serde(rename = "Tp")]
    pub tp: Option<Max35Text>,

    #[serde(rename = "Dt")]
    pub dt: Option<Isodate>,

    #[serde(rename = "Ctry")]
    pub ctry: Option<CountryCode>,

    #[serde(rename = "Cd")]
    pub cd: Option<Max10Text>,

    #[serde(rename = "Amt")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "Inf", default)]
    pub inf: Vec<Max35Text>,
}

impl Validate for StructuredRegulatoryReporting3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct StructuredRemittanceInformation18 {
    #[serde(rename = "RfrdDocInf", default)]
    pub rfrd_doc_inf: Vec<ReferredDocumentInformation8>,

    #[serde(rename = "RfrdDocAmt")]
    pub rfrd_doc_amt: Option<RemittanceAmount4>,

    #[serde(rename = "CdtrRefInf")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,

    #[serde(rename = "Invcr")]
    pub invcr: Option<PartyIdentification272>,

    #[serde(rename = "Invcee")]
    pub invcee: Option<PartyIdentification272>,

    #[serde(rename = "TaxRmt")]
    pub tax_rmt: Option<TaxData1>,

    #[serde(rename = "GrnshmtRmt")]
    pub grnshmt_rmt: Option<Garnishment4>,

    #[serde(rename = "AddtlRmtInf", default)]
    pub addtl_rmt_inf: Vec<Max140Text>,
}

impl Validate for StructuredRemittanceInformation18 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct SupplementaryData1 {
    #[serde(rename = "PlcAndNm")]
    pub plc_and_nm: Option<Max350Text>,

    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1,
}

impl Validate for SupplementaryData1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct SupplementaryDataEnvelope1 {}

impl Validate for SupplementaryDataEnvelope1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxAmount3 {
    #[serde(rename = "Rate")]
    pub rate: Option<PercentageRate>,

    #[serde(rename = "TaxblBaseAmt")]
    pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "TtlAmt")]
    pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "Dtls", default)]
    pub dtls: Vec<TaxRecordDetails3>,
}

impl Validate for TaxAmount3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxAuthorisation1 {
    #[serde(rename = "Titl")]
    pub titl: Option<Max35Text>,

    #[serde(rename = "Nm")]
    pub nm: Option<Max140Text>,
}

impl Validate for TaxAuthorisation1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxData1 {
    #[serde(rename = "Cdtr")]
    pub cdtr: Option<TaxParty1>,

    #[serde(rename = "Dbtr")]
    pub dbtr: Option<TaxParty2>,

    #[serde(rename = "UltmtDbtr")]
    pub ultmt_dbtr: Option<TaxParty2>,

    #[serde(rename = "AdmstnZone")]
    pub admstn_zone: Option<Max35Text>,

    #[serde(rename = "RefNb")]
    pub ref_nb: Option<Max140Text>,

    #[serde(rename = "Mtd")]
    pub mtd: Option<Max35Text>,

    #[serde(rename = "TtlTaxblBaseAmt")]
    pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "TtlTaxAmt")]
    pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,

    #[serde(rename = "Dt")]
    pub dt: Option<Isodate>,

    #[serde(rename = "SeqNb")]
    pub seq_nb: Option<Number>,

    #[serde(rename = "Rcrd", default)]
    pub rcrd: Vec<TaxRecord3>,
}

impl Validate for TaxData1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxParty1 {
    #[serde(rename = "TaxId")]
    pub tax_id: Option<Max35Text>,

    #[serde(rename = "RegnId")]
    pub regn_id: Option<Max35Text>,

    #[serde(rename = "TaxTp")]
    pub tax_tp: Option<Max35Text>,
}

impl Validate for TaxParty1 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxParty2 {
    #[serde(rename = "TaxId")]
    pub tax_id: Option<Max35Text>,

    #[serde(rename = "RegnId")]
    pub regn_id: Option<Max35Text>,

    #[serde(rename = "TaxTp")]
    pub tax_tp: Option<Max35Text>,

    #[serde(rename = "Authstn")]
    pub authstn: Option<TaxAuthorisation1>,
}

impl Validate for TaxParty2 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxPeriod3 {
    #[serde(rename = "Yr")]
    pub yr: Option<Isoyear>,

    #[serde(rename = "Tp")]
    pub tp: Option<TaxRecordPeriod1Code>,

    #[serde(rename = "FrToDt")]
    pub fr_to_dt: Option<DatePeriod2>,
}

impl Validate for TaxPeriod3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxRecord3 {
    #[serde(rename = "Tp")]
    pub tp: Option<Max35Text>,

    #[serde(rename = "Ctgy")]
    pub ctgy: Option<Max35Text>,

    #[serde(rename = "CtgyDtls")]
    pub ctgy_dtls: Option<Max35Text>,

    #[serde(rename = "DbtrSts")]
    pub dbtr_sts: Option<Max35Text>,

    #[serde(rename = "CertId")]
    pub cert_id: Option<Max35Text>,

    #[serde(rename = "FrmsCd")]
    pub frms_cd: Option<Max35Text>,

    #[serde(rename = "Prd")]
    pub prd: Option<TaxPeriod3>,

    #[serde(rename = "TaxAmt")]
    pub tax_amt: Option<TaxAmount3>,

    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Option<Max140Text>,
}

impl Validate for TaxRecord3 {}


#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]

pub struct TaxRecordDetails3 {
    #[serde(rename = "Prd")]
    pub prd: Option<TaxPeriod3>,

    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl Validate for TaxRecordDetails3 {}


#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum TaxRecordPeriod1Code {
    #[serde(rename = "MM01")]
    Mm01,
    #[serde(rename = "MM02")]
    Mm02,
    #[serde(rename = "MM03")]
    Mm03,
    #[serde(rename = "MM04")]
    Mm04,
    #[serde(rename = "MM05")]
    Mm05,
    #[serde(rename = "MM06")]
    Mm06,
    #[serde(rename = "MM07")]
    Mm07,
    #[serde(rename = "MM08")]
    Mm08,
    #[serde(rename = "MM09")]
    Mm09,
    #[serde(rename = "MM10")]
    Mm10,
    #[serde(rename = "MM11")]
    Mm11,
    #[serde(rename = "MM12")]
    Mm12,
    #[serde(rename = "QTR1")]
    Qtr1,
    #[serde(rename = "QTR2")]
    Qtr2,
    #[serde(rename = "QTR3")]
    Qtr3,
    #[serde(rename = "QTR4")]
    Qtr4,
    #[serde(rename = "HLF1")]
    Hlf1,
    #[serde(rename = "HLF2")]
    Hlf2,
    __Unknown__(String),
}

impl Default for TaxRecordPeriod1Code {
    fn default() -> TaxRecordPeriod1Code {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TaxRecordPeriod1Code {}



#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct TrueFalseIndicator (pub bool);

impl Validate for TrueFalseIndicator {}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Uuidv4Identifier (pub String);

impl Validate for Uuidv4Identifier {}
