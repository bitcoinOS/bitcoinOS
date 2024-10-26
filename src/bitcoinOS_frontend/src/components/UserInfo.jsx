import React, { useState, useEffect } from 'react';
import { Box, Image, SimpleGrid, useRadio, useRadioGroup, VStack, Text } from '@chakra-ui/react';

import useGetWalletPool from '../utils/walletActor';
import { Observable } from '@dfinity/agent/lib/cjs/observable';

import { UserInfo } from '../store/useWalletStore';

function RadioCard(props) {
  const { getInputProps, getRadioProps } = useRadio(props);

  const input = getInputProps();
  const checkbox = getRadioProps();

  return (
    <Box as="label">
      <input {...input} />
      <Box
        {...checkbox}
        height='64px'
        width='64px'
        borderWidth={props.isChecked ? '4px' : '2px'}
        borderColor={props.isChecked ? 'blue.500' : 'gray.200'}
        borderRadius="full"
        overflow="hidden"
        cursor="pointer"
        position="relative"
      >
        <Image src={props.src} height='64px' alt="Avatar" objectFit="cover" />
        {props.isChecked && (
          <Box
            position="absolute"
            top="0"
            left="0"
            right="0"
            bottom="0"
            bg="rgba(0, 0, 255, 0.3)"
            display="flex"
            alignItems="center"
            justifyContent="center"
          >
            <Box as="span" color="white" fontSize="2xl">✓</Box>
          </Box>
        )}
      </Box>
    </Box>
  );
}

const AvatarSelector = ({ selectedAvatar, onAvatarSelect }) => {
  const { get_wallets, get_user_image, update_user_info } = useGetWalletPool()

  const { userImageList } = UserInfo();

  useEffect(() => {
    const res = get()
  }, [])

  const get = async () => {
    const res = await get_user_image();
  }

  const avatars = [
    'https://bit.ly/dan-abramov',
    'https://bit.ly/kent-c-dodds',
    'https://bit.ly/ryan-florence',
    'https://bit.ly/prosper-baba',
  ];

  //const [selectedAvatar, setSelectedAvatar] = useState(null);

  useEffect(() => {
    if (userImageList && userImageList.length > 0) {
      onAvatarSelect(userImageList[0].link);
    }
  }, [userImageList]);

  const { getRootProps, getRadioProps } = useRadioGroup({
    name: 'avatar',
    value: selectedAvatar,
    onChange: (value) => {
      console.log('Selected avatar:', value);
      onAvatarSelect(value);
    },
  });

  const group = getRootProps();

  return (
    <VStack spacing={4}>
      <SimpleGrid
        width='340px'
        height='64px'
        columns={4}
        spacing={0}
        {...group}
      >
        {userImageList.map((avatar) => {
          const radio = getRadioProps({ value: avatar.link });
          return (
            <RadioCard
              key={avatar.id}
              src={avatar.link}
              {...radio}
              isChecked={selectedAvatar === avatar.link}
            />
          );
        })}
      </SimpleGrid>
      {/*
      <Text> SelectedAvatar: {selectedAvatar}</Text>
        */}
    </VStack>
  );
};

export default AvatarSelector;