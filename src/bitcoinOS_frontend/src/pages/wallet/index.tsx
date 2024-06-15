import { Box, Flex, Spacer, Button } from '@chakra-ui/react'

import {
  Table,
  Thead,
  Tbody,
  Tfoot,
  Tr,
  Th,
  Td,
  TableCaption,
  TableContainer,
} from '@chakra-ui/react'

import { usePointBackend, PointRecord, Metadata } from "../../ic/PointActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useToast } from '@chakra-ui/react'
import { useEffect, useState, useRef } from 'react';
import { point } from '../../../../declarations/point';


export default function Wallet() {
  const toast = useToast();

  const { actor: pointBackend } = usePointBackend();
  const { identity, login } = useInternetIdentity();

  const [isLogin, setIslogin] = useState<boolean>(false)
  const [isPointInited, setIsPointInited] = useState<boolean>(false)
  const [pointRank, setPointRank] = useState<PointRecord[]>([])

  const btcunity = 100000000n;

  useEffect(() => {
    if (identity) {
      setIslogin(true)
    }
    if (!pointBackend) {
      setIsPointInited(false);
    } else {
      setIsPointInited(true);
    }

  }, [])

  useEffect(() => {
    if (identity) {
      setIslogin(true)
    } else {
      //setIsLoading(false)
      setIslogin(false)
    }
  }, [identity])

  useEffect(() => {
    console.log("------sss")
    console.log(pointBackend)
    if (pointBackend) {
      setIsPointInited(true);
      get_Pointrank()
    } else {
      setIsPointInited(false);
    }
  }, [pointBackend])


  const get_Pointrank = () => {
    console.log('----------goods')
    console.log(pointBackend)
    if (!pointBackend) {
      return
    }
    pointBackend.get_point().then((value: PointRecord[]) => {
      setPointRank(value)
      console.log(":--", pointRank)
    }).catch((error) => {
      toast({
        title: 'Info',
        description: "get stake error",
        status: 'error',
        position: 'top',
        duration: 9000,
        isClosable: true,
      });
      console.error("Error fetching staking pool:", error);
    }).finally(() => {
      console.log("goods")
    });
  }
  return (
    <>
      <Box>
        <Flex justify="center">
          <Flex>
            {pointRank.length > 0 ? (
              <Table variant='simple'>
                <Thead>
                  <Tr>
                    <Th>Rank</Th>
                    <Th>account</Th>
                    <Th isNumeric>staked</Th>
                    <Th>points</Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {pointRank.map((record, index) => (
                    <Tr key={index}>
                      <Td>{index + 1}</Td>
                      <Td>{record.staker.toString()}</Td>
                      <Td isNumeric>{(record.actual_amount / btcunity).toString()}</Td>
                      <Td>{record.points.toString()}</Td>
                    </Tr>
                  ))}
                </Tbody>
              </Table>
            ) : (
              <div>No data available</div>
            )}
          </Flex>
        </Flex>
      </Box>
    </>
  )
}