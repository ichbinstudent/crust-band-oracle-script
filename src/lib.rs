use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm_kit::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    cid: String,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    result: String,
}

const DATA_SOURCE_ID: i64 = 321;
const EXTERNAL_ID: i64 = 321;

#[no_mangle]
fn prepare_impl(input: Input) {
    oei::ask_external_data(
        DATA_SOURCE_ID,
        EXTERNAL_ID,
        format!("{}", input.cid).as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(_input: Input) -> Output {
    Output {
        result: ext::load_majority::<String>(EXTERNAL_ID).unwrap(),
    }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
