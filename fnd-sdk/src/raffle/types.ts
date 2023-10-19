import { PublicKey } from "@solana/web3.js";


export enum RoundState {
    Uninitialized = "uninitialized",
    Initialized = "initialized",
    Open = "open",
    Drawn = "drawn",
}

export interface Round {
    roundState: RoundState;
    roundValue: bigint;
    startedAt: number;
    solAmount: number;
    feeAmount: number;
    participants: number;
    roundEndsAt: number;
    lastTransactedAt: number;
    winner: number;
    publicKey: string;
    
    roundNumber: number;
  }

  export interface RoundSettings {
    completedRounds: number;
    totalSolDeposited: number;
    totalFeeCollected: number;
    totalParticipants: number;
    roundDuration: number;
    minSolToDeposit: number;
    feePercent: number;
    canInitializeNextRound: boolean;
    lastRoundEndsAt: number;
    lastTransactedAt: number;
    publicKey: string;
  }

  export interface UserRound {
    round: string;
    solDeposited: number;
    startSolPosition: number;
    user: string;
    lastTransactedAt: number;
    depositedAt: number;
    publicKey: string;
  }