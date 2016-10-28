// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import util from '../api/util';

export const ERRORS = {
  invalidAddress: 'address is an invalid network address',
  duplicateAddress: 'the address is already in your address book',
  invalidChecksum: 'address has failed the checksum formatting',
  invalidName: 'name should not be blank and longer than 2',
  invalidAbi: 'abi should be a valid JSON array',
  invalidCode: 'code should be the compiled hex string'
};

export function validateAbi (abi, api) {
  let abiError = null;
  let abiParsed = null;

  try {
    abiParsed = JSON.parse(abi);

    if (!api.util.isArray(abiParsed) || !abiParsed.length) {
      abiError = ERRORS.inavlidAbi;
    } else {
      abi = JSON.stringify(abiParsed);
    }
  } catch (error) {
    abiError = ERRORS.invalidAbi;
  }

  return {
    abi,
    abiError,
    abiParsed
  };
}

export function validateAddress (address) {
  let addressError = null;

  if (!address) {
    addressError = ERRORS.invalidAddress;
  } else if (!util.isAddressValid(address)) {
    addressError = ERRORS.invalidAddress;
  } else {
    address = util.toChecksumAddress(address);
  }

  return {
    address,
    addressError
  };
}

export function validateCode (code, api) {
  let codeError = null;

  if (!code.length) {
    codeError = ERRORS.invalidCode;
  } else if (!api.util.isHex(code)) {
    codeError = ERRORS.invalidCode;
  }

  return {
    code,
    codeError
  };
}

export function validateName (name) {
  const nameError = !name || name.trim().length < 2 ? ERRORS.invalidName : null;

  return {
    name,
    nameError
  };
}
