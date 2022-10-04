import { Scope, MissingScopes, Approvals } from "@cartridge/controller";
import {
  Call,
  Abi,
  InvocationsDetails,
  InvokeFunctionResponse,
} from "starknet";
import { StarknetChainId } from "starknet/constants";
import { toBN } from "starknet/dist/utils/number";
import { calculateTransactionHash } from "starknet/utils/hash";
import { fromCallsToExecuteCalldata } from "starknet/utils/transaction";

import Controller, { diff } from "utils/account";
import Storage from "utils/storage";

const execute = (controller: Controller, approvals: Approvals) => async (transactions: Call | Call[], abis?: Abi[], transactionsDetail?: InvocationsDetails, sync?: boolean): Promise<InvokeFunctionResponse> => {
  const calls = Array.isArray(transactions) ? transactions : [transactions];

  const scopes = calls.map(
    (txn) =>
    ({
      target: txn.contractAddress,
      method: txn.entrypoint,
    } as Scope),
  );

  if (sync) {
    const calldata = fromCallsToExecuteCalldata(calls)
    const hash = calculateTransactionHash(controller.address, transactionsDetail.version, calldata, transactionsDetail.maxFee, StarknetChainId.TESTNET, transactionsDetail.nonce);
    await pollForTransaction(hash)
  } else {
    const missing = diff(scopes, approvals.scopes);
    if (missing.length > 0) {
      throw new MissingScopes(missing);
    }
  }

  if (
    approvals.maxFee &&
    transactionsDetail && toBN(transactionsDetail.maxFee).gt(toBN(approvals.maxFee))
  ) {
    throw new Error("transaction fees exceed pre-approved limit")
  }

  return await controller.execute(calls, abis, transactionsDetail);
}


// Three minutes
const TIMEOUT = 1000 * 60 * 3;
const INTERNVAL = 100;

function pollForTransaction(hash: string) {
  return new Promise((resolve, reject) => {
    let elapsed = -100;
    const checkApproval = async () => {
      elapsed += 100;

      if (elapsed > TIMEOUT) {
        clearInterval(timeout);
        return reject("timeout");
      }

      const txn = Storage.get(hash);
      if (txn) {
        Storage.remove(hash)
        clearInterval(timeout);
        return resolve("");
      }
    };

    // Poll for approval
    const timeout = setInterval(checkApproval, INTERNVAL);

    // Call on leading edge
    checkApproval();
  });
}

export default execute;