import {delay, getSecsNano, ICurrencyTokenSetup, makeCurrencyTokenSetup} from "./utils";
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {assert} from 'chai';
import {deleteCurrencyToken, deployCurrencyToken} from "./deploy";

describe('currency token', () => {
    const tokenControllerId = Ed25519KeyIdentity.generate();
    const tokenHolder1Id = Ed25519KeyIdentity.generate();
    const tokenHolder2Id = Ed25519KeyIdentity.generate();

    let tokenController: ICurrencyTokenSetup;
    let tokenHolder1: ICurrencyTokenSetup;
    let tokenHolder2: ICurrencyTokenSetup;

    before(async () => {
        await deployCurrencyToken(tokenControllerId.getPrincipal());

        tokenController = await makeCurrencyTokenSetup(tokenControllerId);
        tokenHolder1 = await makeCurrencyTokenSetup(tokenHolder1Id);
        tokenHolder2 = await makeCurrencyTokenSetup(tokenHolder2Id);

        await Promise.all([
            tokenController.agent.fetchRootKey(),
            tokenHolder1.agent.fetchRootKey(),
            tokenHolder2.agent.fetchRootKey()
        ]);
    });

    after(async () => {
        await deleteCurrencyToken();
    });

    it("recurrent payments work fine", async () => {
        await tokenController.currencyTokenClient.mint({
            entries: [
                {
                    to: tokenHolder1Id.getPrincipal(),
                    qty: 1000n,
                    recurrence: [],
                    event_payload: []
                }
            ]
        });

        await tokenHolder1.currencyTokenClient.transfer({
            entries: [
                {
                    to: tokenHolder2Id.getPrincipal(),
                    qty: 200n,
                    recurrence: [
                        {
                            duration_nano: getSecsNano(10),
                            iterations: {
                                Infinite: null
                            }
                        }
                    ],
                    event_payload: []
                }
            ]
        });

        await delay(1000 * 60);

        const balance1 = await tokenHolder1.currencyTokenClient.get_balance_of({
            account_owner: tokenHolder1Id.getPrincipal()
        });

        assert.equal(balance1.balance, 0n, "Token holder 1 should waste all their money");

        const balance2 = await tokenHolder1.currencyTokenClient.get_balance_of({
            account_owner: tokenHolder2Id.getPrincipal()
        });

        assert.equal(balance2.balance, 1000n, "Token holder 2 balance should become 1000");
    });
});