import {execAsync} from "./utils";
import {Principal} from "@dfinity/principal";

export async function deployMembershipToken(controller: Principal) {
    const command = `dfx deploy membership-token --argument '(record { default_controllers = opt vec { principal "${controller}" }; })'`;

    console.log('Deploying membership token...');
    console.log(command);
    console.log(await execAsync(command));
}

export async function deployCurrencyToken(controller: Principal) {
    const command = `dfx deploy currency-token --argument '(record { info = record { name = "Test Currency"; symbol = "TST"; decimals = 2 : nat8; }; default_controllers = opt vec { principal "${controller}" }; })'`;

    console.log('Deploying currency token...');
    console.log(command)
    console.log(await execAsync(command));
}

export async function deleteMembershipToken() {
    const command = `dfx canister stop membership-token && dfx canister delete membership-token`;

    console.log(command);
    console.log(await execAsync(command));
}

export async function deleteCurrencyToken() {
    const command = `dfx canister stop currency-token && dfx canister delete currency-token`;

    console.log(command);
    console.log(await execAsync(command));
}