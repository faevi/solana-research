export type FungibleTokenPrice = {
  "version": "0.1.0",
  "name": "fungible_token_price",
  "instructions": [
    {
      "name": "testInstruction",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "roundSetting",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "completedRounds",
            "type": "u64"
          },
          {
            "name": "totalSolDeposited",
            "type": "u64"
          },
          {
            "name": "totalFeeCollected",
            "type": "u64"
          },
          {
            "name": "totalParticipants",
            "type": "u64"
          },
          {
            "name": "roundDuration",
            "type": "u64"
          },
          {
            "name": "minSolToDeposit",
            "type": "u64"
          },
          {
            "name": "feePercent",
            "type": "u64"
          },
          {
            "name": "canInitializeNextRound",
            "type": "bool"
          },
          {
            "name": "lastRoundEndsAt",
            "type": "u64"
          },
          {
            "name": "lastTransactedAt",
            "type": "u64"
          },
          {
            "name": "placeholderOne",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "round",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "roundState",
            "type": {
              "defined": "RoundState"
            }
          },
          {
            "name": "roundValue",
            "type": "u64"
          },
          {
            "name": "startedAt",
            "type": "u64"
          },
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "feeAmount",
            "type": "u64"
          },
          {
            "name": "participants",
            "type": "u64"
          },
          {
            "name": "roundEndsAt",
            "type": "u64"
          },
          {
            "name": "lastTransactedAt",
            "type": "u64"
          },
          {
            "name": "winner",
            "type": "publicKey"
          },
          {
            "name": "roundNumber",
            "type": "u64"
          },
          {
            "name": "placeholderOne",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "userRound",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "round",
            "type": "publicKey"
          },
          {
            "name": "solDeposited",
            "type": "u64"
          },
          {
            "name": "startSolPosition",
            "type": "u64"
          },
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "lastTransactedAt",
            "type": "u64"
          },
          {
            "name": "depositedAt",
            "type": "u64"
          },
          {
            "name": "placeholderOne",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "RoundState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Uninitialized"
          },
          {
            "name": "Initialized"
          },
          {
            "name": "Open"
          },
          {
            "name": "Drawn"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "RoundIsNotOpen",
      "msg": "RoundIsNotOpen"
    },
    {
      "code": 6001,
      "name": "RoundIsOpen",
      "msg": "RoundIsOpen"
    },
    {
      "code": 6002,
      "name": "UserDoesntHaveAuthority",
      "msg": "UserDoesntHaveAuthority"
    },
    {
      "code": 6003,
      "name": "LessThenMinDeposit",
      "msg": "LessThenMinDeposit"
    },
    {
      "code": 6004,
      "name": "LastRoundIsOpen",
      "msg": "LastRoundIsOpen"
    },
    {
      "code": 6005,
      "name": "RoundPdaIsIncorrect",
      "msg": "RoundPdaIsIncorrect"
    },
    {
      "code": 6006,
      "name": "WrongRoundNumber",
      "msg": "WrongRoundNumber"
    },
    {
      "code": 6007,
      "name": "UserIsNotWinner",
      "msg": "UserIsNotWinner"
    },
    {
      "code": 6008,
      "name": "RewardsAlreadyClaimed",
      "msg": "RewardsAlreadyClaimed"
    }
  ]
};

export const IDL: FungibleTokenPrice = {
  "version": "0.1.0",
  "name": "fungible_token_price",
  "instructions": [
    {
      "name": "testInstruction",
      "accounts": [
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "roundSetting",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "completedRounds",
            "type": "u64"
          },
          {
            "name": "totalSolDeposited",
            "type": "u64"
          },
          {
            "name": "totalFeeCollected",
            "type": "u64"
          },
          {
            "name": "totalParticipants",
            "type": "u64"
          },
          {
            "name": "roundDuration",
            "type": "u64"
          },
          {
            "name": "minSolToDeposit",
            "type": "u64"
          },
          {
            "name": "feePercent",
            "type": "u64"
          },
          {
            "name": "canInitializeNextRound",
            "type": "bool"
          },
          {
            "name": "lastRoundEndsAt",
            "type": "u64"
          },
          {
            "name": "lastTransactedAt",
            "type": "u64"
          },
          {
            "name": "placeholderOne",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "round",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "roundState",
            "type": {
              "defined": "RoundState"
            }
          },
          {
            "name": "roundValue",
            "type": "u64"
          },
          {
            "name": "startedAt",
            "type": "u64"
          },
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "feeAmount",
            "type": "u64"
          },
          {
            "name": "participants",
            "type": "u64"
          },
          {
            "name": "roundEndsAt",
            "type": "u64"
          },
          {
            "name": "lastTransactedAt",
            "type": "u64"
          },
          {
            "name": "winner",
            "type": "publicKey"
          },
          {
            "name": "roundNumber",
            "type": "u64"
          },
          {
            "name": "placeholderOne",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "userRound",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "round",
            "type": "publicKey"
          },
          {
            "name": "solDeposited",
            "type": "u64"
          },
          {
            "name": "startSolPosition",
            "type": "u64"
          },
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "lastTransactedAt",
            "type": "u64"
          },
          {
            "name": "depositedAt",
            "type": "u64"
          },
          {
            "name": "placeholderOne",
            "type": "publicKey"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "RoundState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Uninitialized"
          },
          {
            "name": "Initialized"
          },
          {
            "name": "Open"
          },
          {
            "name": "Drawn"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "RoundIsNotOpen",
      "msg": "RoundIsNotOpen"
    },
    {
      "code": 6001,
      "name": "RoundIsOpen",
      "msg": "RoundIsOpen"
    },
    {
      "code": 6002,
      "name": "UserDoesntHaveAuthority",
      "msg": "UserDoesntHaveAuthority"
    },
    {
      "code": 6003,
      "name": "LessThenMinDeposit",
      "msg": "LessThenMinDeposit"
    },
    {
      "code": 6004,
      "name": "LastRoundIsOpen",
      "msg": "LastRoundIsOpen"
    },
    {
      "code": 6005,
      "name": "RoundPdaIsIncorrect",
      "msg": "RoundPdaIsIncorrect"
    },
    {
      "code": 6006,
      "name": "WrongRoundNumber",
      "msg": "WrongRoundNumber"
    },
    {
      "code": 6007,
      "name": "UserIsNotWinner",
      "msg": "UserIsNotWinner"
    },
    {
      "code": 6008,
      "name": "RewardsAlreadyClaimed",
      "msg": "RewardsAlreadyClaimed"
    }
  ]
};
