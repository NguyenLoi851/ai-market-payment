use near_sdk::AccountId;
use serde_json::json;

// #[tokio::test]
// async fn test_register_model() -> Result<(), Box<dyn std::error::Error>> {
//     let sandbox = near_workspaces::sandbox().await?;
//     let contract_wasm = near_workspaces::compile_project("./").await?;
//     let contract = sandbox.dev_deploy(&contract_wasm).await?;
//     let first_creator_account = sandbox.dev_create_account().await?;

//     // Check initial model count
//     let count_zero = contract.view("get_model_count").args_json(json!({})).await?;
//     assert_eq!(count_zero.json::<u32>()?, 0);

//     // Register the first model
//     let outcome_register_first_model = first_creator_account
//         .call(contract.id(), "register_model")
//         .args_json(json!({"model_id": 1, "fee_per_prompt": 100}))
//         .transact().await?;
//     assert!(outcome_register_first_model.is_success());

//     // Attempt to register the same model ID again
//     let outcome_register_duplicate_model = first_creator_account
//         .call(contract.id(), "register_model")
//         .args_json(json!({"model_id": 1, "fee_per_prompt": 200})) // Trying to register with the same ID
//         .transact().await?;

//     // Check if the transaction failed
//     assert_eq!(outcome_register_duplicate_model.logs(), [
//         "Model ID already exists. Registration failed.",
//     ]);

//     // Check model count remains unchanged
//     let count_after_duplicate_attempt = contract
//         .view("get_model_count")
//         .args_json(json!({})).await?;
//     assert_eq!(count_after_duplicate_attempt.json::<u32>()?, 1);

//     Ok(())
// }

// #[tokio::test]
// async fn test_model_count_and_info() -> Result<(), Box<dyn std::error::Error>> {
//     let sandbox = near_workspaces::sandbox().await?;
//     let contract_wasm = near_workspaces::compile_project("./").await?;
//     let contract = sandbox.dev_deploy(&contract_wasm).await?;
//     let first_creator_account = sandbox.dev_create_account().await?;

//     // Check initial model count
//     let count_zero = contract.view("get_model_count").args_json(json!({})).await?;
//     assert_eq!(count_zero.json::<u32>()?, 0);

//     // Register the first model
//     let _ = first_creator_account
//         .call(contract.id(), "register_model")
//         .args_json(json!({"model_id": 1, "fee_per_prompt": 100}))
//         .transact().await?;

//     // Check model count after registration
//     let count_one = contract.view("get_model_count").args_json(json!({})).await?;
//     assert_eq!(count_one.json::<u32>()?, 1);

//     // Test getting the model info for the model
//     let model_info = contract.view("get_model_info").args_json(json!({"model_id": 1})).await?;
//     let expected_info =
//         json!({
//         "creator_wallet": first_creator_account.id(),
//         "fee_per_prompt": 100,
//         "usage_count": 0
//     });

//     assert_eq!(model_info.json::<serde_json::Value>()?, expected_info);

//     // Test getting info for a non-existent model
//     let non_existent_model_info = contract
//         .view("get_model_info")
//         .args_json(json!({"model_id": 999})).await?;
//     assert!(non_existent_model_info.json::<serde_json::Value>()?.is_null());

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_model_info() -> Result<(), Box<dyn std::error::Error>> {
//     let sandbox = near_workspaces::sandbox().await?;
//     let contract_wasm = near_workspaces::compile_project("./").await?;
//     let contract = sandbox.dev_deploy(&contract_wasm).await?;
//     let first_creator_account = sandbox.dev_create_account().await?;
//     let second_creator_account = sandbox.dev_create_account().await?;

//     // Register the first model
//     let _ = first_creator_account
//         .call(contract.id(), "register_model")
//         .args_json(json!({"model_id": 1, "fee_per_prompt": 100}))
//         .transact().await?;

//     // Update model info
//     let new_creator_wallet: Option<AccountId> = Some(second_creator_account.id().clone());
//     let new_fee_per_prompt: Option<u32> = Some(150);

//     // Attempt to update model info as a non-creator (should fail)
//     let outcome_unauthorized_update = second_creator_account
//         .call(contract.id(), "update_model_info")
//         .args_json(
//             json!({
//             "model_id": 1,
//             "new_fee_per_prompt": Some(new_fee_per_prompt.unwrap()*2),
//             "new_creator_wallet": new_creator_wallet,
//         })
//         )
//         .transact().await?;
//     assert_eq!(outcome_unauthorized_update.logs(), [
//         "Only the creator can update the model information.",
//     ]);

//     // Update by the creator (should succeed)
//     let outcome_update_model = first_creator_account
//         .call(contract.id(), "update_model_info")
//         .args_json(
//             json!({
//                 "model_id": 1,
//                 "new_fee_per_prompt": new_fee_per_prompt,
//                 "new_creator_wallet": new_creator_wallet,
//             })
//         )
//         .transact().await?;
//     assert!(outcome_update_model.is_success());

//     // Test getting the model info for the updated model
//     let updated_model_info = contract
//         .view("get_model_info")
//         .args_json(json!({"model_id": 1})).await?;
//     let expected_info_after_updating =
//         json!({
//         "creator_wallet": second_creator_account.id(),
//         "fee_per_prompt": new_fee_per_prompt.unwrap(),
//         "usage_count": 0
//     });

//     assert_eq!(updated_model_info.json::<serde_json::Value>()?, expected_info_after_updating);

//     Ok(())
// }

// let transfer_amount = U128::from(parse_near!("100 μN"));

// let outcome = user_account
//         .call(contract.id(), "set_greeting")
//         .args_json(json!({"greeting": "Hello World!"}))
//         .transact()
//         .await?;
//     assert!(outcome.is_success());

//     let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
//     assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

#[tokio::test]
async fn test_ft() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let ft_contract_wasm = std::fs::read("./tests/external-contracts/ft.wasm")?;
    let ft_contract = sandbox.dev_deploy(&ft_contract_wasm).await?;

    let first_user = sandbox.dev_create_account().await?;
    let second_user = sandbox.dev_create_account().await?;

    let first_user_balance = ft_contract.view("ft_balance_of").args_json(json!({"account_id": first_user.id()})).await?;
    println!("first_user_balance: {}", first_user_balance.json::<u32>()?);

    Ok(())
}
