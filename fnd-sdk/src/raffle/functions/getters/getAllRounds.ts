import { web3 } from '@project-serum/anchor';
import { anchorRawBNsAndPubkeysToNumsAndStrings, getFilteredAccounts } from '../../helpers';
import { returnAnchorProgram } from '../../helpers';
import {Round, RoundSettings, UserRound} from '../../types';

export const getAllRounds = async (
  programId: web3.PublicKey,
  connection: web3.Connection,
): Promise<{
  rounds: Round[];
}> => {
  const program = await returnAnchorProgram(programId, connection);

  const rounds = (await program.account.round.all()).map((raw) =>
  anchorRawBNsAndPubkeysToNumsAndStrings(raw),
).map((raw) => ({
  ...raw, roundValue: undefined,
}));

  return {
    rounds,
  };
};
