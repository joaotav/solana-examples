import { Connection, SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import { getSysvarClockCodec } from "@solana/sysvars";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com",
  "confirmed"
);

const accountInfo = await connection.getAccountInfo(SYSVAR_CLOCK_PUBKEY);

// Deserialize the account data
const decodedClock = getSysvarClockCodec().decode(
  new Uint8Array(accountInfo?.data ?? [])
);
console.log(
  JSON.stringify(
    accountInfo,
    (key, value) => {
      if (key === "data" && value && value.length > 1) {
        return [
          value[0],
          "...truncated, total bytes: " + value.length + "...",
          value[value.length - 1]
        ];
      }
      return value;
    },
    2
  )
);
console.log(decodedClock);