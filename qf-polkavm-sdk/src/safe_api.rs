#[macro_export]
macro_rules! safe_api {
    () => {
        $crate::host_functions!();

        const STORAGE_KEY_LEN: usize = 256;
        type StorageKey = [u8; STORAGE_KEY_LEN];
        const READ_BUFFER_LEN: usize = 2048;
        type ReadBuffer = [u8; READ_BUFFER_LEN];

        #[derive(Debug)]
        #[non_exhaustive]
        enum LoadDataError {
            GetDataErr(u64),
            DecodeErr(parity_scale_codec::Error),
        }

        pub fn load_data<T: parity_scale_codec::Decode>() -> Result<T, LoadDataError> {
            let mut buffer = [0u8; READ_BUFFER_LEN];

            match unsafe { get_user_data(&mut buffer as *mut ReadBuffer as u32) } {
                0 => (),
                err => return Err(LoadDataError::GetDataErr(err)),
            };

            T::decode(&mut &buffer[..]).map_err(|err| LoadDataError::DecodeErr(err))
        }

        #[derive(Debug)]
        #[non_exhaustive]
        enum LoadError {
            GetErr(u64),
            DecodeErr(parity_scale_codec::Error),
            KeyErr(KeyError),
        }

        pub fn load<T: parity_scale_codec::Decode>(
            key: &StorageKey,
        ) -> Result<Option<T>, LoadError> {
            let mut buffer = [0u8; READ_BUFFER_LEN];

            // Read from storage
            match unsafe { get(key.as_ptr() as u32, buffer.as_mut_ptr() as u32) } {
                0 => (),
                err => return Err(LoadError::GetErr(err)),
            };

            if buffer.iter().all(|v| *v == 0) {
                return Ok(None);
            }

            match T::decode(&mut &buffer[..]) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(LoadError::DecodeErr(err)),
            }
        }

        #[derive(Debug)]
        #[non_exhaustive]
        enum SaveError {
            NotEnoughtSpaceToEncode,
            SetErr(u64),
        }

        pub fn save<T: parity_scale_codec::Encode>(
            value: &T,
            key: &StorageKey,
        ) -> Result<(), SaveError> {
            let mut buffer = [0u8; READ_BUFFER_LEN];

            // Encode structure
            for (pos, elem) in T::encode(value).iter().enumerate() {
                match buffer.get_mut(pos) {
                    Some(buffer_place) => *buffer_place = *elem,
                    None => return Err(SaveError::NotEnoughtSpaceToEncode),
                }
            }

            // Write to storage
            match unsafe { set(key.as_ptr() as u32, buffer.as_mut_ptr() as u32) } {
                0 => Ok(()),
                err => Err(SaveError::SetErr(err)),
            }
        }

        #[derive(Debug)]
        #[non_exhaustive]
        enum KeyError {
            IsNotAscii,
            ExceedsMaxLength,
        }

        trait TryIntoKey {
            fn try_into_key(self) -> Result<StorageKey, KeyError>;
        }

        impl TryIntoKey for &str {
            fn try_into_key(self) -> Result<StorageKey, KeyError> {
                try_into_key(self)
            }
        }

        pub fn try_into_key(s: &str) -> Result<StorageKey, KeyError> {
            if !s.is_ascii() {
                return Err(KeyError::IsNotAscii);
            }
            if s.len() > STORAGE_KEY_LEN {
                return Err(KeyError::ExceedsMaxLength);
            }

            let mut key = [32u8; STORAGE_KEY_LEN];
            let formatted_key = format!("{s:>width$}", width = STORAGE_KEY_LEN);
            key.copy_from_slice(formatted_key.as_bytes());
            Ok(key)
        }

        struct StorageCell<T> {
            pub data: T,
            pub storage_key: StorageKey,
        }

        impl<T: parity_scale_codec::Encode + parity_scale_codec::Decode> StorageCell<T> {
            pub fn load_from_key<K: TryIntoKey>(storage_key: K) -> Result<Option<Self>, LoadError> {
                let storage_key = storage_key
                    .try_into_key()
                    .map_err(|err| LoadError::KeyErr(err))?;
                match load(&storage_key)? {
                    Some(data) => Ok(Some(Self { data, storage_key })),
                    None => Ok(None),
                }
            }

            pub fn load(&self) -> Result<Option<Self>, LoadError> {
                match load(&self.storage_key)? {
                    Some(data) => Ok(Some(Self {
                        data,
                        storage_key: self.storage_key.clone(),
                    })),
                    None => Ok(None),
                }
            }

            pub fn save(&self) -> Result<(), SaveError> {
                save(&self.data, &self.storage_key)
            }
        }
    };
}
