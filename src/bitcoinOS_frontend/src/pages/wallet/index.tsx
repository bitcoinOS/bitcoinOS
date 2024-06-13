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

import { usePointBackend } from "../../ic/PointActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useToast } from '@chakra-ui/react'
import React, { useEffect, useState, useRef } from 'react';

export default function Wallet() {

  const { actor: pointBackend } = usePointBackend();
  const { identity, login } = useInternetIdentity();

  const [isLogin, setIslogin] = useState<boolean>(false)
  const [isPointInited, setIsPointInited] = useState<boolean>(false)

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
    if (!pointBackend) {
      setIsPointInited(false);
    } else {
      setIsPointInited(true);
    }
  }, [pointBackend])


  const test = () => {
    console.log('----------goods')
    console.log(pointBackend)
  }
  return (
    <>
      <Box>
        <Button onClick={test}>test</Button>
        <Flex justify="center">
          <Flex>
            <Table variant='simple'>
              <TableCaption>Imperial to metric conversion factors</TableCaption>
              <Thead>
                <Tr>
                  <Th>To convert</Th>
                  <Th>into</Th>
                  <Th isNumeric>multiply by</Th>
                </Tr>
              </Thead>
              <Tbody>
                <Tr>
                  <Td>inches</Td>
                  <Td>millimetres (mm)</Td>
                  <Td isNumeric>25.4</Td>
                </Tr>
                <Tr>
                  <Td>feet</Td>
                  <Td>centimetres (cm)</Td>
                  <Td isNumeric>30.48</Td>
                </Tr>
                <Tr>
                  <Td>yards</Td>
                  <Td>metres (m)</Td>
                  <Td isNumeric>0.91444</Td>
                </Tr>
              </Tbody>
              <Tfoot>
                <Tr>
                  <Th>To convert</Th>
                  <Th>into</Th>
                  <Th isNumeric>multiply by</Th>
                </Tr>
              </Tfoot>
            </Table>
          </Flex>
        </Flex>
      </Box>
    </>
  )
}