export type Nitro = {
  "version": "0.1.0",
  "name": "nitro",
  "constants": [
    {
      "name": "CHANNEL",
      "type": "string",
      "value": "\"channel\""
    }
  ],
  "instructions": [
    {
      "name": "init",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "balances",
          "type": {
            "array": [
              "u64",
              2
            ]
          }
        }
      ]
    },
    {
      "name": "join",
      "accounts": [
        {
          "name": "channel",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "update",
      "accounts": [
        {
          "name": "channel",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "finalize",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "challenge",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "distribute",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "channel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "channel",
            "type": "publicKey"
          },
          {
            "name": "channelAuthority",
            "type": "publicKey"
          },
          {
            "name": "liquidityMint",
            "type": "publicKey"
          },
          {
            "name": "liquidityAccount",
            "type": "publicKey"
          },
          {
            "name": "userA",
            "type": "publicKey"
          },
          {
            "name": "userB",
            "type": "publicKey"
          },
          {
            "name": "shareMint",
            "type": "publicKey"
          },
          {
            "name": "userAShareAccount",
            "type": "publicKey"
          },
          {
            "name": "userBShareAccount",
            "type": "publicKey"
          },
          {
            "name": "tokenProgram",
            "type": "publicKey"
          },
          {
            "name": "shareTokenProgram",
            "type": "publicKey"
          },
          {
            "name": "channelAuthorityBump",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          },
          {
            "name": "state",
            "type": {
              "defined": "ChannelState"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ChannelOutcome",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Pending"
          },
          {
            "name": "Open"
          },
          {
            "name": "Finalizing"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidNonce",
      "msg": "Invalid nonce"
    }
  ]
};

export const IDL: Nitro = {
  "version": "0.1.0",
  "name": "nitro",
  "constants": [
    {
      "name": "CHANNEL",
      "type": "string",
      "value": "\"channel\""
    }
  ],
  "instructions": [
    {
      "name": "init",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "balances",
          "type": {
            "array": [
              "u64",
              2
            ]
          }
        }
      ]
    },
    {
      "name": "join",
      "accounts": [
        {
          "name": "channel",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "update",
      "accounts": [
        {
          "name": "channel",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "finalize",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "challenge",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "distribute",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "channel",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The channel containing the users and balance sheet"
          ]
        },
        {
          "name": "channelAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shareMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userAShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userBShareAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "liquidityMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "The token mint which the channel will transact"
          ]
        },
        {
          "name": "liquidityAccount",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "The token account that will custody funds"
          ]
        },
        {
          "name": "funder",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "funderTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "channel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "channel",
            "type": "publicKey"
          },
          {
            "name": "channelAuthority",
            "type": "publicKey"
          },
          {
            "name": "liquidityMint",
            "type": "publicKey"
          },
          {
            "name": "liquidityAccount",
            "type": "publicKey"
          },
          {
            "name": "userA",
            "type": "publicKey"
          },
          {
            "name": "userB",
            "type": "publicKey"
          },
          {
            "name": "shareMint",
            "type": "publicKey"
          },
          {
            "name": "userAShareAccount",
            "type": "publicKey"
          },
          {
            "name": "userBShareAccount",
            "type": "publicKey"
          },
          {
            "name": "tokenProgram",
            "type": "publicKey"
          },
          {
            "name": "shareTokenProgram",
            "type": "publicKey"
          },
          {
            "name": "channelAuthorityBump",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          },
          {
            "name": "state",
            "type": {
              "defined": "ChannelState"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "ChannelOutcome",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Pending"
          },
          {
            "name": "Open"
          },
          {
            "name": "Finalizing"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidNonce",
      "msg": "Invalid nonce"
    }
  ]
};
