export const idlFactory = ({ IDL }) => {
    const MetadataStorageType__1 = IDL.Variant({
        'S3': IDL.Null,
        'Last': IDL.Null,
        'Fleek': IDL.Null,
        'MetaBox': IDL.Null,
    });
    const TokenIndex__1 = IDL.Nat32;
    const MetadataStorageInfo__1 = IDL.Record({
        'url': IDL.Text,
        'thumb': IDL.Text,
    });
    const TokenIdentifier__5 = IDL.Text;
    const AccountIdentifier = IDL.Text;
    const User__3 = IDL.Variant({
        'principal': IDL.Principal,
        'address': AccountIdentifier,
    });
    const AllowanceRequest__1 = IDL.Record({
        'token': TokenIdentifier__5,
        'owner': User__3,
        'spender': IDL.Principal,
    });
    const Balance__2 = IDL.Nat;
    const CommonError__2 = IDL.Variant({
        'InvalidToken': TokenIdentifier__5,
        'Other': IDL.Text,
    });
    const Result__2_2 = IDL.Variant({
        'ok': Balance__2,
        'err': CommonError__2,
    });
    const SubAccount = IDL.Vec(IDL.Nat8);
    const Balance = IDL.Nat;
    const ApproveRequest__1 = IDL.Record({
        'token': TokenIdentifier__5,
        'subaccount': IDL.Opt(SubAccount),
        'allowance': Balance,
        'spender': IDL.Principal,
    });
    const BalanceRequest__1 = IDL.Record({
        'token': TokenIdentifier__5,
        'user': User__3,
    });
    const CommonError__1 = IDL.Variant({
        'InvalidToken': TokenIdentifier__5,
        'Other': IDL.Text,
    });
    const BalanceResponse__1 = IDL.Variant({
        'ok': Balance,
        'err': CommonError__1,
    });
    const MintRequest__1 = IDL.Record({
        'to': User__3,
        'metadata': IDL.Opt(IDL.Vec(IDL.Nat8)),
    });
    const Memo = IDL.Vec(IDL.Nat8);
    const TransferRequest__1 = IDL.Record({
        'to': User__3,
        'token': TokenIdentifier__5,
        'notify': IDL.Bool,
        'from': User__3,
        'memo': Memo,
        'subaccount': IDL.Opt(SubAccount),
        'amount': Balance,
    });
    const TransferResponse__1 = IDL.Variant({
        'ok': Balance,
        'err': IDL.Variant({
            'CannotNotify': AccountIdentifier,
            'InsufficientBalance': IDL.Null,
            'InvalidToken': TokenIdentifier__5,
            'Rejected': IDL.Null,
            'Unauthorized': AccountIdentifier,
            'Other': IDL.Text,
        }),
    });
    const TokenIdentifier__7 = IDL.Text;
    const AccountIdentifier__4 = IDL.Text;
    const Result__2_1 = IDL.Variant({
        'ok': AccountIdentifier__4,
        'err': CommonError__2,
    });
    const Extension__1 = IDL.Text;
    const Metadata__1 = IDL.Variant({
        'fungible': IDL.Record({
            'decimals': IDL.Nat8,
            'metadata': IDL.Opt(IDL.Vec(IDL.Nat8)),
            'name': IDL.Text,
            'symbol': IDL.Text,
        }),
        'nonfungible': IDL.Record({ 'metadata': IDL.Opt(IDL.Vec(IDL.Nat8)) }),
    });
    const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
    const HttpRequest__1 = IDL.Record({
        'url': IDL.Text,
        'method': IDL.Text,
        'body': IDL.Vec(IDL.Nat8),
        'headers': IDL.Vec(HeaderField),
    });
    const HttpResponse__1 = IDL.Record({
        'body': IDL.Vec(IDL.Nat8),
        'headers': IDL.Vec(HeaderField),
        'status_code': IDL.Nat16,
    });
    const Property__1 = IDL.Record({
        'trait_type': IDL.Text,
        'value': IDL.Text,
    });
    const Result__2 = IDL.Variant({ 'ok': Metadata__1, 'err': CommonError__2 });
    const Result_15 = IDL.Variant({ 'ok': Balance__2, 'err': CommonError__2 });
    const Result_14 = IDL.Variant({
        'ok': IDL.Vec(TokenIndex__1),
        'err': CommonError__2,
    });
    const Time__2 = IDL.Int;
    const Listing__2 = IDL.Record({
        'locked': IDL.Opt(Time__2),
        'seller': IDL.Principal,
        'price': IDL.Nat64,
    });
    const Result_13 = IDL.Variant({
        'ok': IDL.Vec(
            IDL.Tuple(TokenIndex__1, IDL.Opt(Listing__2), IDL.Opt(IDL.Vec(IDL.Nat8)))
        ),
        'err': CommonError__2,
    });
    return IDL.Service({
        'acceptCycles': IDL.Func([], [], []),
        'addMetadataStorageType': IDL.Func([IDL.Text], [], ['oneway']),
        'addMetadataUrlMany': IDL.Func(
            [
                IDL.Vec(
                    IDL.Tuple(
                        MetadataStorageType__1,
                        TokenIndex__1,
                        MetadataStorageInfo__1,
                    )
                ),
            ],
            [],
            ['oneway'],
        ),
        'allowance': IDL.Func([AllowanceRequest__1], [Result__2_2], ['query']),
        'approve': IDL.Func([ApproveRequest__1], [IDL.Bool], []),
        'approveAll': IDL.Func(
            [IDL.Vec(ApproveRequest__1)],
            [IDL.Vec(TokenIndex__1)],
            [],
        ),
        'availableCycles': IDL.Func([], [IDL.Nat], ['query']),
        'balance': IDL.Func([BalanceRequest__1], [BalanceResponse__1], ['query']),
        'batchMintNFT': IDL.Func(
            [IDL.Vec(MintRequest__1)],
            [IDL.Vec(TokenIndex__1)],
            [],
        ),
        'batchTransfer': IDL.Func(
            [IDL.Vec(TransferRequest__1)],
            [IDL.Vec(TransferResponse__1)],
            [],
        ),
        'bearer': IDL.Func([TokenIdentifier__7], [Result__2_1], ['query']),
        'clearProperties': IDL.Func([], [], ['oneway']),
        'deleteMetadataStorageType': IDL.Func([IDL.Text], [], ['oneway']),
        'extensions': IDL.Func([], [IDL.Vec(Extension__1)], ['query']),
        'getAllowances': IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(TokenIndex__1, IDL.Principal))],
            ['query'],
        ),
        'getMedataStorageType': IDL.Func([], [IDL.Vec(IDL.Text)], []),
        'getMinter': IDL.Func([], [IDL.Principal], ['query']),
        'getProperties': IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat))))],
            ['query'],
        ),
        'getRegistry': IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(TokenIndex__1, AccountIdentifier__4))],
            ['query'],
        ),
        'getRootBucketId': IDL.Func([], [IDL.Opt(IDL.Text)], []),
        'getScore': IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(TokenIndex__1, IDL.Float64))],
            ['query'],
        ),
        'getStorageMetadataUrl': IDL.Func(
            [MetadataStorageType__1, TokenIndex__1],
            [IDL.Tuple(IDL.Text, IDL.Text)],
            [],
        ),
        'getTokens': IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(TokenIndex__1, Metadata__1))],
            ['query'],
        ),
        'getTokensByIds': IDL.Func(
            [IDL.Vec(TokenIndex__1)],
            [IDL.Vec(IDL.Tuple(TokenIndex__1, Metadata__1))],
            ['query'],
        ),
        'getTokensByProperties': IDL.Func(
            [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Text)))],
            [IDL.Vec(IDL.Tuple(TokenIndex__1, Metadata__1))],
            ['query'],
        ),
        'http_request': IDL.Func([HttpRequest__1], [HttpResponse__1], ['query']),
        'initCap': IDL.Func([], [IDL.Opt(IDL.Text)], []),
        'initLastMetadata': IDL.Func([TokenIndex__1, TokenIndex__1], [], []),
        'initproperties': IDL.Func([TokenIndex__1, TokenIndex__1], [], []),
        'lookProperties': IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(Property__1, IDL.Vec(TokenIndex__1)))],
            ['query'],
        ),
        'lookPropertyScoreByTokenId': IDL.Func(
            [],
            [
                IDL.Vec(
                    IDL.Tuple(TokenIndex__1, IDL.Vec(IDL.Tuple(Property__1, IDL.Int64)))
                ),
            ],
            ['query'],
        ),
        'metadata': IDL.Func([TokenIdentifier__7], [Result__2], ['query']),
        'mintNFT': IDL.Func([MintRequest__1], [TokenIndex__1], []),
        'replaceMetadata': IDL.Func(
            [MetadataStorageType__1, TokenIndex__1, TokenIndex__1],
            [],
            [],
        ),
        'setMinter': IDL.Func([IDL.Principal], [], []),
        'setScoreOfTokenId': IDL.Func([IDL.Int64], [], []),
        'supply': IDL.Func([TokenIdentifier__7], [Result_15], ['query']),
        'tokens': IDL.Func([AccountIdentifier__4], [Result_14], ['query']),
        'tokens_ext': IDL.Func([AccountIdentifier__4], [Result_13], ['query']),
        'transfer': IDL.Func([TransferRequest__1], [TransferResponse__1], []),
        'updateNFTName': IDL.Func(
            [IDL.Text, IDL.Text, TokenIndex__1, TokenIndex__1],
            [],
            [],
        ),
    });
};
export const init = ({ IDL }) => { return []; };