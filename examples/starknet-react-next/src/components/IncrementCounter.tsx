import { useAccount, useStarknetExecute } from "@starknet-react/core";

export function IncrementCounter() {
  const { account } = useAccount();
  const { execute } = useStarknetExecute({
    calls: [
      {
        contractAddress:
          "0x036486801b8f42e950824cba55b2df8cccb0af2497992f807a7e1d9abd2c6ba1",
        entrypoint: "incrementCounter",
        calldata: ["0x1"],
      },
    ],
  });

  if (!account) {
    return null;
  }

  return (
    <div>
      <button onClick={() => {
        try {
          execute();
        } catch (err) {
          console.error(err);
        }
      }}>Increment Counter by 1</button>
    </div>
  );
}
