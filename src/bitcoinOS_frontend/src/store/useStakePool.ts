import { create } from 'zustand';
import { RewardRecord, LeaderBoardStatus } from '../ic/PointActors';

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

interface PointStore {
    pointRank: RewardRecordPoint[];
    creditRank: RewardRecordPoint[];
    rankStatus: LeaderBoardStatus | null;
    setPointRank: (pointRank: RewardRecordPoint[]) => void;
    setCreditRank: (creditRank: RewardRecordPoint[]) => void;
    setRankStatus: (rankStatus: LeaderBoardStatus | null) => void;
}

interface PoolRecords {
    stakeRecords: StakingRecord[];
    setStakeRecords: (stakeRecords: StakingRecord[]) => void;
    allStakeRecords: allStakingRecord[];
    setAllStakeRecords: (allStakeRecords: allStakingRecord[]) => void;
}

export const usePointStore = create<PointStore>((set) => ({
    pointRank: [],
    creditRank: [],
    rankStatus: null,
    setPointRank: (pointRank) => set({ pointRank }),
    setCreditRank: (creditRank) => set({ creditRank }),
    setRankStatus: (rankStatus) => set({ rankStatus }),
}));

export const usePoolrecordStore = create<PoolRecords>((set) => ({
    stakeRecords: [],
    setStakeRecords: (stakeRecords) => set({ stakeRecords }),
    allStakeRecords: [],
    setAllStakeRecords: (allStakeRecords) => set({ allStakeRecords }),
}));