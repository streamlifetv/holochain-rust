use crate::nucleus::ribosome::{api::ZomeApiResult, Runtime};
use holochain_core_types::signature::Provenance;
use holochain_dpki::utils::Verify;
use holochain_wasm_utils::api_serialization::verify_signature::VerifySignatureArgs;
use std::convert::TryFrom;
use wasmi::{RuntimeArgs, RuntimeValue};

/// ZomeApiFunction::VerifySignature function code
/// args: [0] encoded MemoryAllocation as u64
/// Expected argument: u64
/// Returns an HcApiReturnCode as I64
pub fn invoke_verify_signature(runtime: &mut Runtime, args: &RuntimeArgs) -> ZomeApiResult {
    let context = runtime.context()?;

    // deserialize args
    let args_str = runtime.load_json_string_from_args(&args);

    let verification_args = match VerifySignatureArgs::try_from(args_str.clone()) {
        Ok(verify_signature_input) => verify_signature_input,
        // Exit on error
        Err(_) => {
            context.log(format!(
                "err/zome: invoke_verify_signature failed to deserialize SerializedEntry: {:?}",
                args_str
            ));
            return ribosome_error_code!(ArgumentDeserializationFailed);
        }
    };

    let verification_result = verification_args
        .provenance
        .verify(verification_args.payload.clone())
        .unwrap_or(false);

    runtime.store_as_json_string(verification_result)
}
