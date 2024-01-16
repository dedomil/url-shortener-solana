import { Connection, PublicKey } from "@solana/web3.js";
import { Hono } from "hono";
import { AnchorProvider, Program } from "@coral-xyz/anchor";

import { HELIUS_API_KEY, PROGRAM_ID } from "./config";
import IDL from "../idl.json";
import { encode } from "bs58";

const app = new Hono();

const rpcUrl = `https://devnet.helius-rpc.com/?api-key=${HELIUS_API_KEY}`;
const connection = new Connection(rpcUrl);

const program = new Program(
    IDL,
    new PublicKey(PROGRAM_ID),
    new AnchorProvider(connection)
);

app.get("/:shortcode", async (c) => {
    let shortcode = c.req.param("shortcode");

    if (!shortcode) {
        c.status(400);
        return c.json({
            message: "shortcode can't be empty",
        });
    }

    let bytes = encode(Buffer.from(shortcode));

    let offset =
        8 + // discriminator
        32 + // pubkey
        4; // shortcode

    let [account] = await program.account.urlAccount.all([
        { memcmp: { offset, bytes } },
    ]);

    if (!account) {
        c.status(404);
        return c.json({
            message: "not found",
        });
    }

    return c.redirect(account.account.url);
});

app.get("*", async (c) => {
    c.status(404);
    return c.json({
        message: "not found",
    });
});

export default app;
