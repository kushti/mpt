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

import isURL from 'validator/lib/isURL';

import { api } from '../parity';

import { getTokenTotalSupply } from '../utils';

const {
  isHex,
  isAddressValid,
  toChecksumAddress
} = api.util;

export const ADDRESS_TYPE = 'ADDRESS_TYPE';
export const TOKEN_ADDRESS_TYPE = 'TOKEN_ADDRESS_TYPE';
export const SIMPLE_TOKEN_ADDRESS_TYPE = 'SIMPLE_TOKEN_ADDRESS_TYPE';
export const TLA_TYPE = 'TLA_TYPE';
export const SIMPLE_TLA_TYPE = 'SIMPLE_TLA_TYPE';
export const UINT_TYPE = 'UINT_TYPE';
export const STRING_TYPE = 'STRING_TYPE';
export const HEX_TYPE = 'HEX_TYPE';
export const URL_TYPE = 'URL_TYPE';

export const ERRORS = {
  invalidTLA: 'The TLA should be 3 characters long',
  invalidUint: 'Please enter a non-negative integer',
  invalidString: 'Please enter at least a character',
  invalidAccount: 'Please select an account to transact with',
  invalidRecipient: 'Please select an account to send to',
  invalidAddress: 'The address is not in the correct format',
  invalidTokenAddress: 'The address is not a regular token contract address',
  invalidHex: 'Please enter an hexadecimal string (digits and letters from a to z)',
  invalidAmount: 'Please enter a positive amount > 0',
  invalidTotal: 'The amount is greater than the availale balance',
  tlaAlreadyTaken: 'This TLA address is already registered',
  addressAlreadyTaken: 'This Token address is already registered',
  invalidURL: 'Please enter a valid URL'
};

const validateAddress = (address) => {
  if (!isAddressValid(address)) {
    return {
      error: ERRORS.invalidAddress,
      valid: false
    };
  }

  return {
    value: toChecksumAddress(address),
    error: null,
    valid: true
  };
};

const validateTokenAddress = (address, contract, simple) => {
  const addressValidation = validateAddress(address);
  if (!addressValidation.valid) return addressValidation;

  if (simple) return addressValidation;

  return getTokenTotalSupply(address)
    .then(balance => {
      if (balance === null) {
        return {
          error: ERRORS.invalidTokenAddress,
          valid: false
        };
      }

      return contract.instance
        .fromAddress.call({}, [ address ])
        .then(([id, tla, base, name, owner]) => {
          if (owner !== '0x0000000000000000000000000000000000000000') {
            return {
              error: ERRORS.addressAlreadyTaken,
              valid: false
            };
          }
        });
    })
    .then((result) => {
      if (result) return result;
      return addressValidation;
    });
};

const validateTLA = (tla, contract, simple) => {
  if (tla.toString().length !== 3) {
    return {
      error: ERRORS.invalidTLA,
      valid: false
    };
  }

  const fTLA = tla.toString().toUpperCase();

  if (simple) {
    return {
      value: fTLA,
      error: null,
      valid: true
    };
  }

  return contract.instance
    .fromTLA.call({}, [ fTLA ])
    .then(([id, address, base, name, owner]) => {
      if (owner !== '0x0000000000000000000000000000000000000000') {
        return {
          error: ERRORS.tlaAlreadyTaken,
          valid: false
        };
      }
    })
    .then((result) => {
      if (result) return result;
      return {
        value: fTLA,
        error: null,
        valid: true
      };
    });
};

const validateUint = (uint) => {
  if (!/^\d+$/.test(uint) || parseInt(uint) <= 0) {
    return {
      error: ERRORS.invalidUint,
      valid: false
    };
  }

  return {
    value: parseInt(uint),
    error: null,
    valid: true
  };
};

const validateString = (string) => {
  if (string.toString().length === 0) {
    return {
      error: ERRORS.invalidString,
      valid: false
    };
  }

  return {
    value: string.toString(),
    error: null,
    valid: true
  };
};

const validateHex = (string) => {
  if (!isHex(string.toString())) {
    return {
      error: ERRORS.invalidHex,
      valid: false
    };
  }

  return {
    value: string.toString(),
    error: null,
    valid: true
  };
};

const validateURL = (string) => {
  if (!isURL(string.toString())) {
    return {
      error: ERRORS.invalidURL,
      valid: false
    };
  }

  return {
    value: string.toString(),
    error: null,
    valid: true
  };
};

export const validate = (value, type, contract) => {
  if (type === ADDRESS_TYPE) return validateAddress(value);
  if (type === TOKEN_ADDRESS_TYPE) return validateTokenAddress(value, contract);
  if (type === SIMPLE_TOKEN_ADDRESS_TYPE) return validateTokenAddress(value, contract, true);
  if (type === TLA_TYPE) return validateTLA(value, contract);
  if (type === SIMPLE_TLA_TYPE) return validateTLA(value, contract, true);
  if (type === UINT_TYPE) return validateUint(value);
  if (type === STRING_TYPE) return validateString(value);
  if (type === HEX_TYPE) return validateHex(value);
  if (type === URL_TYPE) return validateURL(value);

  return { valid: true, error: null };
};
