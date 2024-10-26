import { Box, Flex, Tooltip, Table, Thead, Tbody, Tr, Th, Td, useToast, Button } from '@chakra-ui/react';
import { usePointBackend, RewardRecord, Metadata } from "../../ic/PointActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useEffect, useState } from 'react';
import { truncateMiddle } from '../../utils/utils'
import { usePointStore } from '../../store/useStakePool';
import { checkIdentityExpiration } from '../../utils/utils';
import { useConnectStore } from '../../store/useConnectStore';

export default function Wallet() {


  return (
    <Box>
      {/*
      <Flex justify="center">
        <Flex>
          {pointRank.length > 0 ? (
            <Table variant="simple" border="1px solid #e2e8f0">
              <Thead>
                <Tr>
                  <Th textTransform="none">Rank</Th>
                  <Th textTransform="none">Wallet/Identity</Th>
                  <Th textTransform="none">Account</Th>
                  <Th textTransform="none">Staked</Th>
                  <Th textTransform="none">Points</Th>
                </Tr>
              </Thead>
              <Tbody>
                
                {matchingRecord && (
                  <Tr fontSize='14px' backgroundColor="#fff9db" _hover={{ backgroundColor: "#e2e8f0" }}>
                    <Td>{matchingRecordIndex + 1}</Td>
                    <Td>{Object.keys(matchingRecord.stake_type)[0]}</Td>
                    <Td>
                      {Object.keys(matchingRecord.stake_type)[0] === 'BTCWallet'
                        ? (
                          <Tooltip label={matchingRecord.wallet.toString()} aria-label="Full account">
                            <span>{truncateMiddle(matchingRecord.wallet.toString(), 5, 5)}</span>
                          </Tooltip>
                        )
                        : (
                          <Tooltip label={matchingRecord.staker.toString()} aria-label="Full account">
                            <span>{truncateMiddle(matchingRecord.staker.toString(), 5, 5)}</span>
                          </Tooltip>
                        )
                      }
                    </Td>
                    <Td>{(Number(matchingRecord.actual_amount) / btcunity).toString()} btc</Td>
                    <Td>{matchingRecord.points.toString()}</Td>
                  </Tr>
                )}
                {pointRank.map((record, index) => (
                  <Tr key={index} _hover={{ backgroundColor: "#f7fafc" }} fontSize='14px'>
                    <Td>{index + 1}</Td>
                    <Td>{Object.keys(record.stake_type)[0]}</Td>
                    <Td>
                      {Object.keys(record.stake_type)[0] === 'BTCWallet'
                        ? (
                          <Tooltip label={record.wallet.toString()} aria-label="Full account">
                            <span>{truncateMiddle(record.wallet.toString(), 5, 5)}</span>
                          </Tooltip>
                        )
                        :
                        (
                          <Tooltip label={record.staker.toString()} aria-label="Full account">
                            <span>{truncateMiddle(record.staker.toString(), 5, 5)}</span>
                          </Tooltip>
                        )
                      }
                    </Td>
                    <Td>{(Number(record.actual_amount) / btcunity).toString()} btc</Td>
                    <Td>{record.points.toString()}</Td>
                  </Tr>
                ))}
              </Tbody>
            </Table>
          ) : (
            <div>No Rank available</div>
          )}
        </Flex>
      </Flex>
      */}
      123
    </Box>
  );
}