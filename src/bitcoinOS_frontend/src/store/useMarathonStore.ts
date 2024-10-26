import { create } from 'zustand';
import { RewardRecord, BoxRecord, BoxRewardRecord, StakeRewardRecord, BoxRecordResponse, InviteRewardRecord } from '../ic/PointActors';
import { UserInfo } from '../ic/OsActors';

import { StakingRecord } from '../ic/WalletActors';
import { StakingRecord as allStakingRecord } from '../ic/StakePoolActors'

interface RewardRecordPoint {
    created_at: string;
    id: number;
    point: number;
    point_rank: number;
    point_type: number;
    update_at: string;
    user_id: string;
}

interface UserStore {
    userInfo: UserInfo | null;
    userReward: null;
    userNftReward: null;
    userCredit: RewardRecordPoint | null;
    inviteInfo: InviteRewardRecord | null;
    userStake: StakeRewardRecord | null;
    setUserInfo: (userInfo: UserInfo | null) => void;
    setUserReward: (userReward: null) => void;
    setUserNftReward: (userNftReward: null) => void;
    setUserCredit: (userCredit: RewardRecordPoint | null) => void;
    setInviteInfo: (inviteInfo: InviteRewardRecord | null) => void;
    setUserStake: (userStake: StakeRewardRecord | null) => void;
}

interface BoxStore {
    boxNum: Number;
    boxRecord: BoxRecord[];
    remainingTimes: number[];
    boxReward: BoxRecordResponse | null;
    setBoxNum: (boxNum: Number) => void;
    setBoxRecord: (boxRecord: BoxRecord[]) => void;
    setRemainingTimes: (remainingTimes: number[]) => void;
    updateRemainingTime: (index: number, time: number) => void;
    setBoxReward: (boxReward: BoxRecordResponse | null) => void;
}

export const userStore = create<UserStore>((set) => ({
    userInfo: null,
    userReward: null,
    userNftReward: null,
    userCredit: null,
    inviteInfo: null,
    userStake: null,
    setUserInfo: (userInfo) => set({ userInfo }),
    setUserReward: (userReward) => set({ userReward }),
    setUserNftReward: (userNftReward) => set({ userNftReward }),
    setUserCredit: (userCredit) => set({ userCredit }),
    setInviteInfo: (inviteInfo) => set({ inviteInfo }),
    setUserStake: (userStake) => set({ userStake }),
}));

export const boxStore = create<BoxStore>((set) => ({
    boxNum: 0,
    boxRecord: [],
    remainingTimes: [],
    boxReward: null,
    setBoxNum: (boxNum) => set({ boxNum }),
    setBoxRecord: (boxRecord) => set({ boxRecord }),
    setRemainingTimes: (remainingTimes) => set({ remainingTimes }),
    updateRemainingTime: (index, time) => set((state) => {
        const newRemainingTimes = [...state.remainingTimes];
        newRemainingTimes[index] = time;
        return { remainingTimes: newRemainingTimes };
    }),
    setBoxReward: (boxReward: BoxRecordResponse | null) => set({ boxReward }),
}));