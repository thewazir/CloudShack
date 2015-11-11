use phf;

#[derive(Debug)]
pub struct Band<'a> {
    pub name: &'a str,
    pub start: f32,
    pub end: f32
}

#[derive(Debug)]
pub struct LegacyMode<'a> {
    pub mode: &'a str,
    pub submode: &'a str
}

#[derive(Debug)]
pub struct DxccEntity<'a> {
    pub id: u32,
    pub name: &'a str,
    pub flag: &'a str
}

pub static BANDS: phf::Map<&'static str, Band<'static>> = phf_map!{
    "2190m" => Band { name: "2190m", start: 0.136, end: 0.137 },
    "560m" => Band { name: "560m", start: 0.501, end: 0.504 },
    "160m" => Band { name: "160m", start: 1.8, end: 2.0 },
    "80m" => Band { name: "80m", start: 3.5, end: 4.0 },
    "60m" => Band { name: "60m", start: 5.102, end: 5.404 },
    "40m" => Band { name: "40m", start: 7.0, end: 7.3 },
    "30m" => Band { name: "30m", start: 10.0, end: 10.15 },
    "20m" => Band { name: "20m", start: 14.0, end: 14.35 },
    "17m" => Band { name: "17m", start: 18.068, end: 18.168 },
    "15m" => Band { name: "15m", start: 21.0, end: 21.45 },
    "12m" => Band { name: "12m", start: 24.890, end: 24.99 },
    "10m" => Band { name: "10m", start: 28.0, end: 29.7 },
    "6m" => Band { name: "6m", start: 50.0, end: 54.0 },
    "4m" => Band { name: "4m", start: 70.0, end: 71.0 },
    "2m" => Band { name: "2m", start: 144.0, end: 148.0 },
    "1.25m" => Band { name: "1.25m", start: 222.0, end: 225.0 },
    "70cm" => Band { name: "70cm", start: 420.0, end: 450.0 },
    "33cm" => Band { name: "33cm", start: 902.0, end: 928.0 },
    "23cm" => Band { name: "23cm", start: 1240.0, end: 1300.0 },
    "13cm" => Band { name: "13cm", start: 2300.0, end: 2450.0 },
    "9cm" => Band { name: "9cm", start: 3300.0, end: 3500.0 },
    "6cm" => Band { name: "6cm", start: 5650.0, end: 5925.0 },
    "3cm" => Band { name: "3cm", start: 10000.0, end: 10500.0 },
    "1.25cm" => Band { name: "1.25cm", start: 24000.0, end: 24250.0 },
    "6mm" => Band { name: "6mm", start: 47000.0, end: 47200.0 },
    "4mm" => Band { name: "4mm", start: 75500.0, end: 81000.0 },
    "2.5mm" => Band { name: "2.5mm", start: 119980.0, end: 120020.0 },
    "2mm" => Band { name: "2mm", start: 142000.0, end: 149000.0 },
    "1mm" => Band { name: "1mm", start: 241000.0, end: 250000.0 }
};

pub fn find_band<'a>(freq: f32) -> Option<&'static Band<'static>> {
    for (_, band) in BANDS.into_iter() {
        if band.start <= freq && freq <= band.end {
            return Some(band)
        }
    }
    None
}

pub static LEGACY_MODES: phf::Map<&'static str, LegacyMode<'static>> = phf_map!{
    "AMTORFEC" => LegacyMode { mode: "TOR", submode: "AMTORFEC" },
    "ASCI" => LegacyMode { mode: "RTTY", submode: "ASCI" },
    "CHIP64" => LegacyMode { mode: "CHIP", submode: "CHIP64" },
    "CHIP128" => LegacyMode { mode: "CHIP", submode: "CHIP128" },
    "DOMINOF" => LegacyMode { mode: "DOMINO", submode: "DOMINOF" },
    "FMHELL" => LegacyMode { mode: "HELL", submode: "FMHELL" },
    "FSK31" => LegacyMode { mode: "PSK", submode: "FSK31" },
    "GTOR" => LegacyMode { mode: "TOR", submode: "GTOR" },
    "HELL80" => LegacyMode { mode: "HELL", submode: "HELL80" },
    "HFSK" => LegacyMode { mode: "HELL", submode: "HFSK" },
    "JT4A" => LegacyMode { mode: "JT4", submode: "JT4A" },
    "JT4B" => LegacyMode { mode: "JT4", submode: "JT4B" },
    "JT4C" => LegacyMode { mode: "JT4", submode: "JT4C" },
    "JT4D" => LegacyMode { mode: "JT4", submode: "JT4D" },
    "JT4E" => LegacyMode { mode: "JT4", submode: "JT4E" },
    "JT4F" => LegacyMode { mode: "JT4", submode: "JT4F" },
    "JT4G" => LegacyMode { mode: "JT4", submode: "JT4G" },
    "JT65A" => LegacyMode { mode: "JT65", submode: "JT65A" },
    "JT65B" => LegacyMode { mode: "JT65", submode: "JT65B" },
    "JT65C" => LegacyMode { mode: "JT65", submode: "JT65C" },
    "MFSK8" => LegacyMode { mode: "MFSK", submode: "MFSK8" },
    "MFSK16" => LegacyMode { mode: "MFSK", submode: "MFSK16" },
    "PAC2" => LegacyMode { mode: "PAC", submode: "PAC2" },
    "PAC3" => LegacyMode { mode: "PAC", submode: "PAC3" },
    "PAX2" => LegacyMode { mode: "PAX", submode: "PAX2" },
    "PCW" => LegacyMode { mode: "CW", submode: "PCW" },
    "PSK10" => LegacyMode { mode: "PSK", submode: "PSK10" },
    "PSK31" => LegacyMode { mode: "PSK", submode: "PSK31" },
    "PSK63" => LegacyMode { mode: "PSK", submode: "PSK63" },
    "PSK63F" => LegacyMode { mode: "PSK", submode: "PSK63F" },
    "PSK125" => LegacyMode { mode: "PSK", submode: "PSK125" },
    "PSKAM10" => LegacyMode { mode: "PSK", submode: "PSKAM10" },
    "PSKAM31" => LegacyMode { mode: "PSK", submode: "PSKAM31" },
    "PSKAM50" => LegacyMode { mode: "PSK", submode: "PSKAM50" },
    "PSKFEC31" => LegacyMode { mode: "PSK", submode: "PSKFEC31" },
    "PSKHELL" => LegacyMode { mode: "HELL", submode: "PSKHELL" },
    "QPSK31" => LegacyMode { mode: "PSK", submode: "QPSK31" },
    "QPSK63" => LegacyMode { mode: "PSK", submode: "QPSK63" },
    "QPSK125" => LegacyMode { mode: "PSK", submode: "QPSK125" },
    "THRBX" => LegacyMode { mode: "THRB", submode: "THRBX" }
};

pub fn find_legacy_mode<'a>(mode: &str) -> Option<&'static LegacyMode<'static>> {
    LEGACY_MODES.get(mode)
}

pub static DXCC_ENTITIES: [Option<&'static DxccEntity<'static>>; 522] = [
    None,
    Some(&DxccEntity { id: 1, name: "CANADA", flag: "CA" }),
    None,
    Some(&DxccEntity { id: 3, name: "AFGHANISTAN", flag: "AF" }),
    Some(&DxccEntity { id: 4, name: "AGALEGA & ST BRANDON", flag: "MP" }),
    Some(&DxccEntity { id: 5, name: "ALAND IS", flag: "AX" }),
    Some(&DxccEntity { id: 6, name: "ALASKA", flag: "US" }),
    Some(&DxccEntity { id: 7, name: "ALBANIA", flag: "AL" }),
    Some(&DxccEntity { id: 8, name: "ALDABRA", flag: "SC" }),
    Some(&DxccEntity { id: 9, name: "AMERICAN SAMOA", flag: "AS" }),
    Some(&DxccEntity { id: 10, name: "AMSTERDAM & ST PAUL", flag: "FR" }),
    Some(&DxccEntity { id: 11, name: "ANDAMAN & NICOBAR IS", flag: "IN" }),
    Some(&DxccEntity { id: 12, name: "ANGUILLA", flag: "AI" }),
    Some(&DxccEntity { id: 13, name: "ANTARCTICA", flag: "AQ" }),
    Some(&DxccEntity { id: 14, name: "ARMENIA", flag: "AM" }),
    Some(&DxccEntity { id: 15, name: "ASIATIC RUSSIA", flag: "RU" }),
    Some(&DxccEntity { id: 16, name: "AUCKLAND & CAMPBELL", flag: "NZ" }),
    Some(&DxccEntity { id: 17, name: "AVES ISLAND", flag: "VE" }),
    Some(&DxccEntity { id: 18, name: "AZERBAIJAN", flag: "AZ" }),
    Some(&DxccEntity { id: 19, name: "BAJO NUEVO", flag: "CO" }),
    Some(&DxccEntity { id: 20, name: "BAKER, HOWLAND IS", flag: "US" }),
    Some(&DxccEntity { id: 21, name: "BALEARIC IS", flag: "ES" }),
    Some(&DxccEntity { id: 22, name: "PALAU", flag: "PW" }),
    Some(&DxccEntity { id: 23, name: "BLENHEIM REEF", flag: "MU" }),
    Some(&DxccEntity { id: 24, name: "BOUVET", flag: "NO" }),
    Some(&DxccEntity { id: 25, name: "BRITISH N. BORNEO", flag: "GB" }),
    Some(&DxccEntity { id: 26, name: "BRITISH SOMALI", flag: "GB" }),
    Some(&DxccEntity { id: 27, name: "BELARUS", flag: "BY" }),
    None,
    Some(&DxccEntity { id: 29, name: "CANARY IS", flag: "IC" }),
    Some(&DxccEntity { id: 30, name: "CELEBE/MOLUCCA IS", flag: "ID" }),
    Some(&DxccEntity { id: 31, name: "C KIRIBATI", flag: "KI" }),
    Some(&DxccEntity { id: 32, name: "CEUTA & MELILLA", flag: "ES" }),
    Some(&DxccEntity { id: 33, name: "CHAGOS", flag: "GB" }),
    Some(&DxccEntity { id: 34, name: "CHATHAM IS", flag: "NZ" }),
    Some(&DxccEntity { id: 35, name: "CHRISTMAS IS", flag: "CX" }),
    Some(&DxccEntity { id: 36, name: "CLIPPERTON IS", flag: "FR" }),
    Some(&DxccEntity { id: 37, name: "COCOS ISLAND", flag: "CR" }),
    Some(&DxccEntity { id: 38, name: "COCOS-KEELING IS", flag: "CC" }),
    None,
    Some(&DxccEntity { id: 40, name: "CRETE", flag: "GR" }),
    Some(&DxccEntity { id: 41, name: "CROZET", flag: "FR" }),
    None,
    Some(&DxccEntity { id: 43, name: "DESECHEO IS", flag: "PR" }),
    None,
    Some(&DxccEntity { id: 45, name: "DODECANESE", flag: "GR" }),
    Some(&DxccEntity { id: 46, name: "EAST MALAYSIA", flag: "MY" }),
    Some(&DxccEntity { id: 47, name: "EASTER IS", flag: "CL" }),
    Some(&DxccEntity { id: 48, name: "EASTERN KIRIBATI", flag: "KI" }),
    Some(&DxccEntity { id: 49, name: "EQUATORIAL GUINEA", flag: "GQ" }),
    Some(&DxccEntity { id: 50, name: "MEXICO", flag: "MX" }),
    Some(&DxccEntity { id: 51, name: "ERITREA", flag: "ER" }),
    Some(&DxccEntity { id: 52, name: "ESTONIA", flag: "EE" }),
    Some(&DxccEntity { id: 53, name: "ETHIOPIA", flag: "ET" }),
    Some(&DxccEntity { id: 54, name: "EUROPEAN RUSSIA", flag: "RU" }),
    Some(&DxccEntity { id: 55, name: "FARQUHAR", flag: "SC" }),
    Some(&DxccEntity { id: 56, name: "FERNANDO DE NORONHA", flag: "BR" }),
    Some(&DxccEntity { id: 57, name: "FRENCH EQ. AFRICA", flag: "FR" }),
    Some(&DxccEntity { id: 58, name: "FRENCH INDO-CHINA", flag: "FR" }),
    Some(&DxccEntity { id: 59, name: "FRENCH WEST AFRICA", flag: "FR" }),
    Some(&DxccEntity { id: 60, name: "BAHAMAS", flag: "BS" }),
    Some(&DxccEntity { id: 61, name: "FRANZ JOSEF LAND", flag: "RU" }),
    Some(&DxccEntity { id: 62, name: "BARBADOS", flag: "BB" }),
    Some(&DxccEntity { id: 63, name: "FRENCH GUIANA", flag: "FR" }),
    Some(&DxccEntity { id: 64, name: "BERMUDA", flag: "BM" }),
    Some(&DxccEntity { id: 65, name: "BRITISH VIRGIN IS", flag: "VG" }),
    Some(&DxccEntity { id: 66, name: "BELIZE", flag: "BZ" }),
    Some(&DxccEntity { id: 67, name: "FRENCH INDIA", flag: "FR" }),
    None,
    Some(&DxccEntity { id: 69, name: "CAYMAN ISLANDS", flag: "KY" }),
    Some(&DxccEntity { id: 70, name: "CUBA", flag: "CU" }),
    Some(&DxccEntity { id: 71, name: "GALAPAGOS", flag: "EC" }),
    Some(&DxccEntity { id: 72, name: "DOMINICAN REPUBLIC", flag: "DO" }),
    None,
    Some(&DxccEntity { id: 74, name: "EL SALVADOR", flag: "SV" }),
    Some(&DxccEntity { id: 75, name: "GEORGIA", flag: "GE" }),
    Some(&DxccEntity { id: 76, name: "GUATEMALA", flag: "GT" }),
    Some(&DxccEntity { id: 77, name: "GRENADA", flag: "GD" }),
    Some(&DxccEntity { id: 78, name: "HAITI", flag: "HT" }),
    Some(&DxccEntity { id: 79, name: "GUADELOUPE", flag: "FR" }),
    Some(&DxccEntity { id: 80, name: "HONDURAS", flag: "HN" }),
    Some(&DxccEntity { id: 81, name: "GERMANY", flag: "DE" }),
    Some(&DxccEntity { id: 82, name: "JAMAICA", flag: "JM" }),
    None,
    Some(&DxccEntity { id: 84, name: "MARTINIQUE", flag: "MQ" }),
    None,
    Some(&DxccEntity { id: 86, name: "NICARAGUA", flag: "NI" }),
    None,
    Some(&DxccEntity { id: 88, name: "PANAMA", flag: "PA" }),
    Some(&DxccEntity { id: 89, name: "TURKS & CAICOS IS", flag: "TC" }),
    Some(&DxccEntity { id: 90, name: "TRINIDAD & TOBAGO", flag: "TT" }),
    Some(&DxccEntity { id: 91, name: "ARUBA", flag: "AW" }),
    None,
    None,
    Some(&DxccEntity { id: 94, name: "ANTIGUA & BARBUDA", flag: "AG" }),
    Some(&DxccEntity { id: 95, name: "DOMINICA", flag: "DM" }),
    Some(&DxccEntity { id: 96, name: "MONTSERRAT", flag: "MS" }),
    Some(&DxccEntity { id: 97, name: "ST LUCIA", flag: "LC" }),
    Some(&DxccEntity { id: 98, name: "ST VINCENT", flag: "VC" }),
    Some(&DxccEntity { id: 99, name: "GLORIOSO IS", flag: "FR" }),
    Some(&DxccEntity { id: 100, name: "ARGENTINA", flag: "AR" }),
    None,
    None,
    Some(&DxccEntity { id: 103, name: "GUAM", flag: "GU" }),
    Some(&DxccEntity { id: 104, name: "BOLIVIA", flag: "BO" }),
    Some(&DxccEntity { id: 105, name: "GUANTANAMO BAY", flag: "US" }),
    Some(&DxccEntity { id: 106, name: "GUERNSEY", flag: "GG" }),
    Some(&DxccEntity { id: 107, name: "GUINEA", flag: "GN" }),
    Some(&DxccEntity { id: 108, name: "BRAZIL", flag: "BR" }),
    Some(&DxccEntity { id: 109, name: "GUINEA-BISSAU", flag: "GW" }),
    Some(&DxccEntity { id: 110, name: "HAWAII", flag: "US" }),
    Some(&DxccEntity { id: 111, name: "HEARD IS", flag: "AU" }),
    Some(&DxccEntity { id: 112, name: "CHILE", flag: "CL" }),
    None,
    Some(&DxccEntity { id: 114, name: "ISLE OF MAN", flag: "IM" }),
    Some(&DxccEntity { id: 115, name: "ITALIAN SOMALI", flag: "IT" }),
    Some(&DxccEntity { id: 116, name: "COLOMBIA", flag: "CO" }),
    Some(&DxccEntity { id: 117, name: "ITU HQ", flag: "CH" }),
    Some(&DxccEntity { id: 118, name: "JAN MAYEN", flag: "NO" }),
    None,
    Some(&DxccEntity { id: 120, name: "ECUADOR", flag: "EC" }),
    None,
    Some(&DxccEntity { id: 122, name: "JERSEY", flag: "JE" }),
    Some(&DxccEntity { id: 123, name: "JOHNSTON IS", flag: "US" }),
    Some(&DxccEntity { id: 124, name: "JUAN DE NOVA", flag: "FR" }),
    Some(&DxccEntity { id: 125, name: "JUAN FERNANDEZ", flag: "CL" }),
    Some(&DxccEntity { id: 126, name: "KALININGRAD", flag: "RU" }),
    None,
    None,
    Some(&DxccEntity { id: 129, name: "GUYANA", flag: "GY" }),
    Some(&DxccEntity { id: 130, name: "KAZAKHSTAN", flag: "KZ" }),
    Some(&DxccEntity { id: 131, name: "KERGUELEN", flag: "FR" }),
    Some(&DxccEntity { id: 132, name: "PARAGUAY", flag: "PY" }),
    Some(&DxccEntity { id: 133, name: "KERMADEC", flag: "NZ" }),
    Some(&DxccEntity { id: 134, name: "KINGMAN REEF", flag: "US" }),
    Some(&DxccEntity { id: 135, name: "KYRGYZSTAN", flag: "KG" }),
    Some(&DxccEntity { id: 136, name: "PERU", flag: "PE" }),
    Some(&DxccEntity { id: 137, name: "REPUBLIC OF KOREA", flag: "KR" }),
    Some(&DxccEntity { id: 138, name: "KURE ISLAND", flag: "US" }),
    None,
    Some(&DxccEntity { id: 140, name: "SURINAME", flag: "SR" }),
    Some(&DxccEntity { id: 141, name: "FALKLAND IS", flag: "FK" }),
    Some(&DxccEntity { id: 142, name: "LAKSHADWEEP ISLANDS", flag: "IN" }),
    Some(&DxccEntity { id: 143, name: "LAOS", flag: "LA" }),
    Some(&DxccEntity { id: 144, name: "URUGUAY", flag: "UY" }),
    Some(&DxccEntity { id: 145, name: "LATVIA", flag: "LV" }),
    Some(&DxccEntity { id: 146, name: "LITHUANIA", flag: "LT" }),
    Some(&DxccEntity { id: 147, name: "LORD HOWE IS", flag: "AU" }),
    Some(&DxccEntity { id: 148, name: "VENEZUELA", flag: "VE" }),
    Some(&DxccEntity { id: 149, name: "AZORES", flag: "PT" }),
    Some(&DxccEntity { id: 150, name: "AUSTRALIA", flag: "AU" }),
    None,
    Some(&DxccEntity { id: 152, name: "MACAO", flag: "MO" }),
    Some(&DxccEntity { id: 153, name: "MACQUARIE IS", flag: "AU" }),
    None,
    None,
    None,
    Some(&DxccEntity { id: 157, name: "NAURU", flag: "NR" }),
    Some(&DxccEntity { id: 158, name: "VANUATU", flag: "VU" }),
    Some(&DxccEntity { id: 159, name: "MALDIVES", flag: "MV" }),
    Some(&DxccEntity { id: 160, name: "TONGA", flag: "TO" }),
    Some(&DxccEntity { id: 161, name: "MALPELO IS", flag: "CO" }),
    Some(&DxccEntity { id: 162, name: "NEW CALEDONIA", flag: "NC" }),
    Some(&DxccEntity { id: 163, name: "PAPUA NEW GUINEA", flag: "PG" }),
    None,
    Some(&DxccEntity { id: 165, name: "MAURITIUS IS", flag: "MU" }),
    Some(&DxccEntity { id: 166, name: "MARIANA IS", flag: "US" }),
    Some(&DxccEntity { id: 167, name: "MARKET REEF", flag: "SE" }),
    Some(&DxccEntity { id: 168, name: "MARSHALL IS", flag: "MH" }),
    Some(&DxccEntity { id: 169, name: "MAYOTTE", flag: "YT" }),
    Some(&DxccEntity { id: 170, name: "NEW ZEALAND", flag: "NZ" }),
    Some(&DxccEntity { id: 171, name: "MELLISH REEF", flag: "AU" }),
    Some(&DxccEntity { id: 172, name: "PITCAIRN IS", flag: "PN" }),
    Some(&DxccEntity { id: 173, name: "MICRONESIA", flag: "FM" }),
    Some(&DxccEntity { id: 174, name: "MIDWAY IS", flag: "US" }),
    Some(&DxccEntity { id: 175, name: "FRENCH POLYNESIA", flag: "PF" }),
    Some(&DxccEntity { id: 176, name: "FIJI", flag: "FJ" }),
    Some(&DxccEntity { id: 177, name: "MINAMI TORISHIMA", flag: "JP" }),
    None,
    Some(&DxccEntity { id: 179, name: "MOLDOVA", flag: "MD" }),
    Some(&DxccEntity { id: 180, name: "MOUNT ATHOS", flag: "GR" }),
    Some(&DxccEntity { id: 181, name: "MOZAMBIQUE", flag: "MZ" }),
    Some(&DxccEntity { id: 182, name: "NAVASSA IS", flag: "US" }),
    None,
    None,
    Some(&DxccEntity { id: 185, name: "SOLOMON ISLANDS", flag: "SB" }),
    None,
    Some(&DxccEntity { id: 187, name: "NIGER", flag: "NE" }),
    Some(&DxccEntity { id: 188, name: "NIUE", flag: "NU" }),
    Some(&DxccEntity { id: 189, name: "NORFOLK IS", flag: "NF" }),
    Some(&DxccEntity { id: 190, name: "SAMOA", flag: "WS" }),
    Some(&DxccEntity { id: 191, name: "N COOK IS", flag: "NZ" }),
    Some(&DxccEntity { id: 192, name: "OGASAWARA", flag: "JP" }),
    None,
    None,
    Some(&DxccEntity { id: 195, name: "ANNOBON I.", flag: "GQ" }),
    None,
    Some(&DxccEntity { id: 197, name: "PALMYRA & JARVIS IS", flag: "US" }),
    None,
    Some(&DxccEntity { id: 199, name: "PETER I IS", flag: "NO" }),
    None,
    Some(&DxccEntity { id: 201, name: "PRINCE EDWARD & MARION", flag: "ZA" }),
    Some(&DxccEntity { id: 202, name: "PUERTO RICO", flag: "PR" }),
    Some(&DxccEntity { id: 203, name: "ANDORRA", flag: "AD" }),
    Some(&DxccEntity { id: 204, name: "REVILLAGIGEDO", flag: "MX" }),
    Some(&DxccEntity { id: 205, name: "ASCENSION ISLAND", flag: "GB" }),
    Some(&DxccEntity { id: 206, name: "AUSTRIA", flag: "AT" }),
    Some(&DxccEntity { id: 207, name: "RODRIGUEZ IS", flag: "MU" }),
    None,
    Some(&DxccEntity { id: 209, name: "BELGIUM", flag: "BE" }),
    None,
    Some(&DxccEntity { id: 211, name: "SABLE ISLAND", flag: "CA" }),
    Some(&DxccEntity { id: 212, name: "BULGARIA", flag: "BG" }),
    Some(&DxccEntity { id: 213, name: "SAINT MARTIN", flag:"FR" }),
    Some(&DxccEntity { id: 214, name: "CORSICA", flag: "FR" }),
    Some(&DxccEntity { id: 215, name: "CYPRUS", flag: "CY" }),
    Some(&DxccEntity { id: 216, name: "SAN ANDRES & PROVIDENCIA", flag: "NI" }),
    Some(&DxccEntity { id: 217, name: "SAN FELIX", flag: "CL" }),
    None,
    Some(&DxccEntity { id: 219, name: "SAO TOME & PRINCIPE", flag: "ST" }),
    None,
    Some(&DxccEntity { id: 221, name: "DENMARK", flag: "DK" }),
    Some(&DxccEntity { id: 222, name: "FAROE IS", flag: "FO" }),
    Some(&DxccEntity { id: 223, name: "ENGLAND", flag: "england" }),
    Some(&DxccEntity { id: 224, name: "FINLAND", flag: "FI" }),
    Some(&DxccEntity { id: 225, name: "SARDINIA", flag: "IT" }),
    None,
    Some(&DxccEntity { id: 227, name: "FRANCE", flag: "FR" }),
    None,
    Some(&DxccEntity { id: 229, name: "GERMAN DEM. REP.", flag: "DE" }),
    Some(&DxccEntity { id: 230, name: "FED REP OF GERMANY", flag: "DE" }),
    None,
    Some(&DxccEntity { id: 232, name: "SOMALIA", flag: "SO" }),
    Some(&DxccEntity { id: 233, name: "GIBRALTAR", flag: "GI" }),
    Some(&DxccEntity { id: 234, name: "S COOK IS", flag: "GS" }),
    Some(&DxccEntity { id: 235, name: "SOUTH GEORGIA IS", flag: "GS" }),
    Some(&DxccEntity { id: 236, name: "GREECE", flag: "GR" }),
    Some(&DxccEntity { id: 237, name: "GREENLAND", flag: "GL" }),
    Some(&DxccEntity { id: 238, name: "SOUTH ORKNEY IS", flag: "GB" }),
    Some(&DxccEntity { id: 239, name: "HUNGARY", flag: "HU" }),
    Some(&DxccEntity { id: 240, name: "SOUTH SANDWICH ISLANDS", flag: "GS" }),
    Some(&DxccEntity { id: 241, name: "SOUTH SHETLAND ISLANDS", flag: "GB" }),
    Some(&DxccEntity { id: 242, name: "ICELAND", flag: "IS" }),
    None,
    None,
    Some(&DxccEntity { id: 245, name: "IRELAND", flag: "IE" }),
    Some(&DxccEntity { id: 246, name: "SOV MILITARY ORDER OF MALTA", flag: "MT" }),
    Some(&DxccEntity { id: 247, name: "SPRATLY IS", flag: "PH" }),
    Some(&DxccEntity { id: 248, name: "ITALY", flag: "IT" }),
    Some(&DxccEntity { id: 249, name: "ST KITTS & NEVIS", flag: "KN" }),
    Some(&DxccEntity { id: 250, name: "ST HELENA IS", flag: "SH" }),
    Some(&DxccEntity { id: 251, name: "LIECHTENSTEIN", flag: "LI" }),
    Some(&DxccEntity { id: 252, name: "ST PAUL ISLAND", flag: "CA" }),
    Some(&DxccEntity { id: 253, name: "ST. PETER & ST. PAUL ROCKS", flag: "BR" }),
    Some(&DxccEntity { id: 254, name: "LUXEMBOURG", flag: "LU" }),
    Some(&DxccEntity { id: 255, name: "SINT MAARTEN, SABA, ST EUSTATIUS", flag: "NL" }),
    Some(&DxccEntity { id: 256, name: "MADEIRA IS", flag: "PT" }),
    Some(&DxccEntity { id: 257, name: "MALTA", flag: "MT" }),
    None,
    Some(&DxccEntity { id: 259, name: "SVALBARD IS", flag: "NO" }),
    Some(&DxccEntity { id: 260, name: "MONACO", flag: "MC" }),
    None,
    Some(&DxccEntity { id: 262, name: "TAJIKISTAN", flag: "TJ" }),
    Some(&DxccEntity { id: 263, name: "NETHERLANDS", flag: "NL" }),
    None,
    Some(&DxccEntity { id: 265, name: "NORTHERN IRELAND", flag: "GB" }),
    Some(&DxccEntity { id: 266, name: "NORWAY", flag: "NO" }),
    None,
    None,
    Some(&DxccEntity { id: 269, name: "POLAND", flag: "PL" }),
    Some(&DxccEntity { id: 270, name: "TOKELAU IS", flag: "TK" }),
    None,
    Some(&DxccEntity { id: 272, name: "PORTUGAL", flag: "PT" }),
    Some(&DxccEntity { id: 273, name: "TRINDADE & MARTIN VAZ ISLANDS", flag: "BR" }),
    Some(&DxccEntity { id: 274, name: "TRISTAN DA CUNHA & GOUGH IS", flag: "GB" }),
    Some(&DxccEntity { id: 275, name: "ROMANIA", flag: "RO" }),
    Some(&DxccEntity { id: 276, name: "TROMELIN", flag: "FR" }),
    Some(&DxccEntity { id: 277, name: "ST PIERRE & MIQUELON", flag: "CA" }),
    Some(&DxccEntity { id: 278, name: "SAN MARINO", flag: "SM" }),
    Some(&DxccEntity { id: 279, name: "SCOTLAND", flag: "scotland" }),
    Some(&DxccEntity { id: 280, name: "TURKMENISTAN", flag: "TM" }),
    Some(&DxccEntity { id: 281, name: "SPAIN", flag: "ES" }),
    Some(&DxccEntity { id: 282, name: "TUVALU", flag: "TV" }),
    Some(&DxccEntity { id: 283, name: "UK BASES ON CYPRUS", flag: "CY" }),
    Some(&DxccEntity { id: 284, name: "SWEDEN", flag: "SE" }),
    Some(&DxccEntity { id: 285, name: "US VIRGIN ISLANDS", flag: "VI" }),
    Some(&DxccEntity { id: 286, name: "UGANDA", flag: "UG" }),
    Some(&DxccEntity { id: 287, name: "SWITZERLAND", flag: "CH" }),
    Some(&DxccEntity { id: 288, name: "UKRAINE", flag: "UA" }),
    Some(&DxccEntity { id: 289, name: "UNITED NATIONS HQ", flag: "united-nations" }),
    None,
    Some(&DxccEntity { id: 291, name: "UNITED STATES", flag: "US" }),
    Some(&DxccEntity { id: 292, name: "UZBEKISTAN", flag: "UZ" }),
    Some(&DxccEntity { id: 293, name: "VIETNAM", flag: "VN" }),
    Some(&DxccEntity { id: 294, name: "WALES", flag: "wales" }),
    Some(&DxccEntity { id: 295, name: "VATICAN", flag: "VA" }),
    Some(&DxccEntity { id: 296, name: "SERBIA", flag: "RS" }),
    Some(&DxccEntity { id: 297, name: "WAKE IS", flag: "US" }),
    Some(&DxccEntity { id: 298, name: "WALLIS & FUTUNA", flag: "WF" }),
    Some(&DxccEntity { id: 299, name: "WEST MALAYSIA", flag: "MY" }),
    None,
    Some(&DxccEntity { id: 301, name: "W KIRIBATI", flag: "KI" }),
    Some(&DxccEntity { id: 302, name: "WESTERN SAHARA", flag: "EH" }),
    Some(&DxccEntity { id: 303, name: "WILLIS IS", flag: "AU" }),
    Some(&DxccEntity { id: 304, name: "BAHRAIN", flag: "BH" }),
    Some(&DxccEntity { id: 305, name: "BANGLADESH", flag: "BD" }),
    Some(&DxccEntity { id: 306, name: "BHUTAN", flag: "BT" }),
    None,
    Some(&DxccEntity { id: 308, name: "COSTA RICA", flag: "CR" }),
    Some(&DxccEntity { id: 309, name: "MYANMAR", flag: "MM" }),
    None,
    None,
    Some(&DxccEntity { id: 312, name: "CAMBODIA", flag: "KH" }),
    None,
    None,
    Some(&DxccEntity { id: 315, name: "SRI LANKA", flag: "LK" }),
    None,
    None,
    Some(&DxccEntity { id: 318, name: "CHINA", flag: "CN" }),
    None,
    None,
    Some(&DxccEntity { id: 321, name: "HONG KONG", flag: "HK" }),
    None,
    None,
    Some(&DxccEntity { id: 324, name: "INDIA", flag: "IN" }),
    None,
    None,
    Some(&DxccEntity { id: 327, name: "INDONESIA", flag: "ID" }),
    None,
    None,
    Some(&DxccEntity { id: 330, name: "IRAN", flag: "IR" }),
    None,
    None,
    Some(&DxccEntity { id: 333, name: "IRAQ", flag: "IQ" }),
    None,
    None,
    Some(&DxccEntity { id: 336, name: "ISRAEL", flag: "IL" }),
    None,
    None,
    Some(&DxccEntity { id: 339, name: "JAPAN", flag: "JP" }),
    None,
    None,
    Some(&DxccEntity { id: 342, name: "JORDAN", flag: "JO" }),
    None,
    Some(&DxccEntity { id: 344, name: "DEMOCRATIC PEOPLE'S REPUBLIC OF KOREA", flag: "KP" }),
    Some(&DxccEntity { id: 345, name: "BRUNEI", flag: "BN" }),
    None,
    None,
    Some(&DxccEntity { id: 348, name: "KUWAIT", flag: "KW" }),
    None,
    None,
    None,
    None,
    None,
    Some(&DxccEntity { id: 354, name: "LEBANON", flag: "LB" }),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(&DxccEntity { id: 363, name: "MONGOLIA", flag: "MN" }),
    None,
    None,
    None,
    None,
    None,
    Some(&DxccEntity { id: 369, name: "NEPAL", flag: "NP" }),
    Some(&DxccEntity { id: 370, name: "OMAN", flag: "OM" }),
    None,
    Some(&DxccEntity { id: 372, name: "PAKISTAN", flag: "PK" }),
    None,
    None,
    Some(&DxccEntity { id: 375, name: "PHILIPPINES", flag: "PH" }),
    Some(&DxccEntity { id: 376, name: "QATAR", flag: "QA" }),
    None,
    Some(&DxccEntity { id: 378, name: "SAUDI ARABIA", flag: "SA" }),
    Some(&DxccEntity { id: 379, name: "SEYCHELLES", flag: "SC" }),
    None,
    Some(&DxccEntity { id: 381, name: "SINGAPORE", flag: "SG" }),
    Some(&DxccEntity { id: 382, name: "DJIBOUTI", flag: "DJ" }),
    None,
    Some(&DxccEntity { id: 384, name: "SYRIA", flag: "SY" }),
    None,
    Some(&DxccEntity { id: 386, name: "TAIWAN", flag: "TW" }),
    Some(&DxccEntity { id: 387, name: "THAILAND", flag: "TH" }),
    None,
    None,
    Some(&DxccEntity { id: 390, name: "TURKEY", flag: "TR" }),
    Some(&DxccEntity { id: 391, name: "UNITED ARAB EMIRATES", flag: "AE" }),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(&DxccEntity { id: 400, name: "ALGERIA", flag: "DZ" }),
    Some(&DxccEntity { id: 401, name: "ANGOLA", flag: "AO" }),
    Some(&DxccEntity { id: 402, name: "BOTSWANA", flag: "BW" }),
    None,
    Some(&DxccEntity { id: 404, name: "BURUNDI", flag: "BI" }),
    None,
    Some(&DxccEntity { id: 406, name: "CAMEROON", flag: "CM" }),
    None,
    Some(&DxccEntity { id: 408, name: "CENTRAL AFRICAN REPUBLIC", flag: "CF" }),
    Some(&DxccEntity { id: 409, name: "CAPE VERDE", flag: "CV" }),
    Some(&DxccEntity { id: 410, name: "CHAD", flag: "TD" }),
    Some(&DxccEntity { id: 411, name: "COMOROS", flag: "KM" }),
    Some(&DxccEntity { id: 412, name: "REPUBLIC OF THE CONGO", flag: "CG" }),
    None,
    Some(&DxccEntity { id: 414, name: "DEM. REPUBLIC OF THE CONGO", flag: "CD" }),
    None,
    Some(&DxccEntity { id: 416, name: "BENIN", flag: "BJ" }),
    None,
    None,
    None,
    Some(&DxccEntity { id: 420, name: "GABON", flag: "GA" }),
    None,
    Some(&DxccEntity { id: 422, name: "THE GAMBIA", flag: "GM" }),
    None,
    Some(&DxccEntity { id: 424, name: "GHANA", flag: "GH" }),
    None,
    None,
    None,
    Some(&DxccEntity { id: 428, name: "COTE D'IVOIRE", flag: "CI" }),
    None,
    Some(&DxccEntity { id: 430, name: "KENYA", flag: "KE" }),
    None,
    Some(&DxccEntity { id: 432, name: "LESOTHO", flag: "LS" }),
    None,
    Some(&DxccEntity { id: 434, name: "LIBERIA", flag: "LR" }),
    None,
    Some(&DxccEntity { id: 436, name: "LIBYA", flag: "LY" }),
    None,
    Some(&DxccEntity { id: 438, name: "MADAGASCAR", flag: "MG" }),
    None,
    Some(&DxccEntity { id: 440, name: "MALAWI", flag: "MW" }),
    None,
    Some(&DxccEntity { id: 442, name: "MALI", flag: "ML" }),
    None,
    Some(&DxccEntity { id: 444, name: "MAURITANIA", flag: "MR" }),
    None,
    Some(&DxccEntity { id: 446, name: "MOROCCO", flag: "MA" }),
    None,
    None,
    None,
    Some(&DxccEntity { id: 450, name: "NIGERIA", flag: "NG" }),
    None,
    Some(&DxccEntity { id: 452, name: "ZIMBABWE", flag: "ZW" }),
    Some(&DxccEntity { id: 453, name: "REUNION", flag: "FR" }),
    Some(&DxccEntity { id: 454, name: "RWANDA", flag: "RW" }),
    None,
    Some(&DxccEntity { id: 456, name: "SENEGAL", flag: "SN" }),
    None,
    Some(&DxccEntity { id: 458, name: "SIERRA LEONE", flag: "SL" }),
    None,
    Some(&DxccEntity { id: 460, name: "ROTUMA IS", flag: "FJ" }),
    None,
    Some(&DxccEntity { id: 462, name: "REPUBLIC OF SOUTH AFRICA", flag: "ZA" }),
    None,
    Some(&DxccEntity { id: 464, name: "NAMIBIA", flag: "NA" }),
    None,
    Some(&DxccEntity { id: 466, name: "SUDAN", flag: "SD" }),
    None,
    Some(&DxccEntity { id: 468, name: "SWAZILAND", flag: "SZ" }),
    None,
    Some(&DxccEntity { id: 470, name: "TANZANIA", flag: "TZ" }),
    None,
    None,
    None,
    Some(&DxccEntity { id: 474, name: "TUNISIA", flag: "TN" }),
    None,
    None,
    None,
    Some(&DxccEntity { id: 478, name: "EGYPT", flag: "EG" }),
    None,
    Some(&DxccEntity { id: 480, name: "BURKINA-FASO", flag: "BF" }),
    None,
    Some(&DxccEntity { id: 482, name: "ZAMBIA", flag: "ZM" }),
    Some(&DxccEntity { id: 483, name: "TOGO", flag: "TG" }),
    None,
    None,
    None,
    None,
    None,
    Some(&DxccEntity { id: 489, name: "CONWAY REEF", flag: "FJ" }),
    Some(&DxccEntity { id: 490, name: "BANABA ISLAND", flag: "KI" }),
    None,
    Some(&DxccEntity { id: 492, name: "YEMEN", flag: "YE" }),
    None,
    None,
    None,
    None,
    Some(&DxccEntity { id: 497, name: "CROATIA", flag: "HR" }),
    None,
    Some(&DxccEntity { id: 499, name: "SLOVENIA", flag: "SI" }),
    None,
    Some(&DxccEntity { id: 501, name: "BOSNIA-HERZEGOVINA", flag: "BA" }),
    Some(&DxccEntity { id: 502, name: "MACEDONIA", flag: "MK" }),
    Some(&DxccEntity { id: 503, name: "CZECH REPUBLIC", flag: "CZ" }),
    Some(&DxccEntity { id: 504, name: "SLOVAK REPUBLIC", flag: "SK" }),
    Some(&DxccEntity { id: 505, name: "PRATAS IS", flag: "TW" }),
    Some(&DxccEntity { id: 506, name: "SCARBOROUGH REEF", flag: "PH" }),
    Some(&DxccEntity { id: 507, name: "TEMOTU PROVINCE", flag: "SB" }),
    Some(&DxccEntity { id: 508, name: "AUSTRAL IS", flag: "PF" }),
    Some(&DxccEntity { id: 509, name: "MARQUESAS IS", flag: "FR" }),
    Some(&DxccEntity { id: 510, name: "PALESTINE", flag: "PS" }),
    Some(&DxccEntity { id: 511, name: "TIMOR-LESTE", flag: "TL" }),
    Some(&DxccEntity { id: 512, name: "CHESTERFIELD IS", flag: "GB" }),
    Some(&DxccEntity { id: 513, name: "DUCIE IS", flag: "PN" }),
    Some(&DxccEntity { id: 514, name: "MONTENEGRO", flag: "ME" }),
    Some(&DxccEntity { id: 515, name: "SWAINS ISLAND", flag: "US" }),
    Some(&DxccEntity { id: 516, name: "ST. BARTHELEMY", flag: "FR" }),
    Some(&DxccEntity { id: 517, name: "CURACAO", flag: "CW" }),
    Some(&DxccEntity { id: 518, name: "SINT MAARTEN", flag: "NL" }),
    Some(&DxccEntity { id: 519, name: "ST EUSTATIUS AND SABA", flag: "AN" }),
    Some(&DxccEntity { id: 520, name: "BONAIRE", flag: "NL" }),
    Some(&DxccEntity { id: 521, name: "SOUTH SUDAN", flag: "SS" })
];

pub fn get_flag(id: usize) -> Option<&'static str> {
    if let Some(entry) = DXCC_ENTITIES.get(id) {
        entry.map(|e| e.flag)
    }
    else {
        None
    }
}