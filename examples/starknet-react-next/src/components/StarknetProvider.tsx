import { sepolia } from "@starknet-react/chains";
import { Connector, StarknetConfig, starkscan } from "@starknet-react/core";
import { PropsWithChildren } from "react";
import CartridgeConnector from "@cartridge/connector";
import { RpcProvider } from "starknet";
import { DOJO_ACTION_ADDRESS } from "./DojoSpawnAndMove";

const ETH_TOKEN_ADDRESS =
  "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";

export function StarknetProvider({ children }: PropsWithChildren) {
  return (
    <StarknetConfig
      autoConnect
      chains={[sepolia]}
      connectors={connectors}
      explorer={starkscan}
      provider={(_chain) =>
        new RpcProvider({
          nodeUrl: process.env.NEXT_PUBLIC_RPC_SEPOLIA,
        })
      }
    >
      {children}
    </StarknetConfig>
  );
}

const url =
  !process.env.NEXT_PUBLIC_VERCEL_BRANCH_URL ||
  process.env.NEXT_PUBLIC_VERCEL_BRANCH_URL.split(".")[0] ===
    "cartridge-starknet-react-next"
    ? process.env.XFRAME_URL
    : "https://" +
      (process.env.NEXT_PUBLIC_VERCEL_BRANCH_URL ?? "").replace(
        "cartridge-starknet-react-next",
        "keychain",
      );

const connectors = [
  new CartridgeConnector(
    [
      {
        target: ETH_TOKEN_ADDRESS,
        method: "approve",
      },
      {
        target: ETH_TOKEN_ADDRESS,
        method: "transfer",
      },
      {
        target: DOJO_ACTION_ADDRESS,
        method: "spawn",
      },
      {
        target: DOJO_ACTION_ADDRESS,
        method: "move",
      },
    ],
    {
      url,
      theme: {
        colors: {
          // e.g. button bg
          primary: "#00b4d8",
          // e.g. button bg hover
          secondary: {
            dark: "red",
            light: "green",
          },
        },
      },
    },
  ) as never as Connector,
];
