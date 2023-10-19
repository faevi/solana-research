import { web3, Program, AnchorProvider } from "@project-serum/anchor";
import { createFakeWallet } from "../common";
import { FungibleTokenPrice, IDL } from "./idl/solana-research";
import { now } from 'lodash';
import { bs58 } from '@project-serum/anchor/dist/cjs/utils/bytes';

type ReturnAnchorProgram = (programId: web3.PublicKey, connection: web3.Connection) => Program<FungibleTokenPrice>;
export const returnAnchorProgram: ReturnAnchorProgram = (programId, connection) =>
  new Program<FungibleTokenPrice>(
    IDL as any,
    programId,
    new AnchorProvider(connection, createFakeWallet(), AnchorProvider.defaultOptions()),
  );

export const nowInSeconds = () => Math.floor(now() / 1000);

export const anchorRawBNsAndPubkeysToNumsAndStrings = (rawAccount: any) => {
  const copyRawAccount = { ...rawAccount };
  const newAccount = parseRawAccount(rawAccount.account);
  return { ...newAccount, publicKey: copyRawAccount.publicKey.toBase58() };
};

const parseRawAccount = (rawAccount: any) => {
  const copyRawAccount = { ...rawAccount };
  for (let key in copyRawAccount) {
    if (copyRawAccount[key] === null || copyRawAccount[key] === undefined || key === "roundValue") continue;
    if (copyRawAccount[key].toNumber) {
      copyRawAccount[key] = copyRawAccount[key].toNumber();
    }

    if (copyRawAccount[key].toBase58) {
      copyRawAccount[key] = copyRawAccount[key].toBase58();
    }
    if (typeof copyRawAccount[key] === 'object' && Object.keys(copyRawAccount[key]).length === 1) {
      copyRawAccount[key] = Object.keys(copyRawAccount[key])[0];
    } else if (typeof copyRawAccount[key] === 'object') {
      copyRawAccount[key] = parseRawAccount(copyRawAccount[key]);
    }
  }
  return copyRawAccount;
};

export const enumToAnchorEnum = (anyEnum: any) => ({ [anyEnum]: {} });

export const getFilteredAccounts = async <T>(
  program: Program<FungibleTokenPrice>,
  accountName: string,
  offset: number,
  indexes: number[],
): Promise<T[]> => {
  return (
    await Promise.all(
      indexes.map((i) =>
        program.account[accountName].all([
          {
            memcmp: {
              offset: offset, // number of bytes
              bytes: bs58.encode(Buffer.from([i])), //PerpetualActive
            },
          },
        ]),
      ),
    )
  ).flat() as any;
};