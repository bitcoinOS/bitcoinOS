export const idlFactory = ({ IDL }) => {
  const TokenIdentifier = IDL.Text;
  const AccountIdentifier = IDL.Text;
  const User = IDL.Variant({
    'principal': IDL.Principal,
    'address': AccountIdentifier,
  });
  const AllowanceRequest = IDL.Record({
    'token': TokenIdentifier,
    'owner': User,
    'spender': IDL.Principal,
  });
  const Balance__1 = IDL.Nat;
  const CommonError = IDL.Variant({
    'InvalidToken': TokenIdentifier,
    'Other': IDL.Text,
  });
  const Result = IDL.Variant({ 'ok': Balance__1, 'err': CommonError });
  const SubAccount = IDL.Vec(IDL.Nat8);
  const Balance = IDL.Nat;
  const ApproveRequest = IDL.Record({
    'token': TokenIdentifier,
    'subaccount': IDL.Opt(SubAccount),
    'allowance': Balance,
    'spender': IDL.Principal,
  });
  const BalanceRequest = IDL.Record({
    'token': TokenIdentifier,
    'user': User,
  });
  const CommonError__1 = IDL.Variant({
    'InvalidToken': TokenIdentifier,
    'Other': IDL.Text,
  });
  const BalanceResponse = IDL.Variant({
    'ok': Balance,
    'err': CommonError__1,
  });
  const TokenIdentifier__1 = IDL.Text;
  const AccountIdentifier__1 = IDL.Text;
  const Result_3 = IDL.Variant({
    'ok': AccountIdentifier__1,
    'err': CommonError,
  });
  const User__1 = IDL.Variant({
    'principal': IDL.Principal,
    'address': AccountIdentifier,
  });
  const Extension = IDL.Text;
  const TokenIndex = IDL.Nat32;
  const Metadata = IDL.Variant({
    'fungible': IDL.Record({
      'decimals': IDL.Nat8,
      'metadata': IDL.Opt(IDL.Vec(IDL.Nat8)),
      'name': IDL.Text,
      'symbol': IDL.Text,
    }),
    'nonfungible': IDL.Record({ 'metadata': IDL.Opt(IDL.Vec(IDL.Nat8)) }),
  });
  const Result_2 = IDL.Variant({ 'ok': TokenIndex, 'err': CommonError });
  const Result_1 = IDL.Variant({ 'ok': Metadata, 'err': CommonError });
  const MintRequest = IDL.Record({
    'to': User,
    'metadata': IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Memo = IDL.Vec(IDL.Nat8);
  const TransferRequest = IDL.Record({
    'to': User,
    'token': TokenIdentifier,
    'notify': IDL.Bool,
    'from': User,
    'memo': Memo,
    'subaccount': IDL.Opt(SubAccount),
    'amount': Balance,
  });
  const TransferResponse = IDL.Variant({
    'ok': Balance,
    'err': IDL.Variant({
      'CannotNotify': AccountIdentifier,
      'InsufficientBalance': IDL.Null,
      'InvalidToken': TokenIdentifier,
      'Rejected': IDL.Null,
      'Unauthorized': AccountIdentifier,
      'Other': IDL.Text,
    }),
  });
  const BifiPalNFT = IDL.Service({
    'acceptCycles': IDL.Func([], [], []),
    'allowance': IDL.Func([AllowanceRequest], [Result], ['query']),
    'approve': IDL.Func([ApproveRequest], [], []),
    'availableCycles': IDL.Func([], [IDL.Nat], ['query']),
    'balance': IDL.Func([BalanceRequest], [BalanceResponse], ['query']),
    'bearer': IDL.Func([TokenIdentifier__1], [Result_3], ['query']),
    'disribute': IDL.Func([User__1], [], []),
    'extensions': IDL.Func([], [IDL.Vec(Extension)], ['query']),
    'freeGift': IDL.Func([AccountIdentifier__1], [IDL.Opt(TokenIndex)], []),
    'getAllowances': IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(TokenIndex, IDL.Principal))],
      ['query'],
    ),
    'getBuyers': IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(AccountIdentifier__1, IDL.Vec(TokenIndex)))],
      ['query'],
    ),
    'getMinted': IDL.Func([], [TokenIndex], ['query']),
    'getMinter': IDL.Func([], [IDL.Principal], ['query']),
    'getRegistry': IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(TokenIndex, AccountIdentifier__1))],
      ['query'],
    ),
    'getSold': IDL.Func([], [TokenIndex], ['query']),
    'getTokens': IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(TokenIndex, Metadata))],
      ['query'],
    ),
    'get_aid': IDL.Func([IDL.Principal], [AccountIdentifier__1], ['query']),
    'get_token_hash': IDL.Func(
      [IDL.Principal, IDL.Nat32],
      [IDL.Text],
      ['query'],
    ),
    'index': IDL.Func([TokenIdentifier__1], [Result_2], ['query']),
    'metadata': IDL.Func([TokenIdentifier__1], [Result_1], ['query']),
    'mintNFT': IDL.Func([MintRequest], [TokenIndex], []),
    'setMinter': IDL.Func([IDL.Principal], [], []),
    'supply': IDL.Func([TokenIdentifier__1], [Result], ['query']),
    'transfer': IDL.Func([TransferRequest], [TransferResponse], []),
  });
  return BifiPalNFT;
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
