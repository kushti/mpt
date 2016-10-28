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

import { registry as registryAbi } from '../../contracts/abi';

import { api } from './parity.js';
import * as addresses from './addresses/actions.js';
import * as accounts from './Accounts/actions.js';
import * as lookup from './Lookup/actions.js';
import * as events from './Events/actions.js';
import * as names from './Names/actions.js';
import * as records from './Records/actions.js';

export { addresses, accounts, lookup, events, names, records };

export const setContract = (contract) => ({ type: 'set contract', contract });

export const fetchContract = () => (dispatch) =>
  api.ethcore.registryAddress()
  .then((address) => {
    const contract = api.newContract(registryAbi, address);
    dispatch(setContract(contract));
    dispatch(fetchFee());
    dispatch(fetchOwner());
  })
  .catch((err) => {
    console.error('could not fetch contract');
    if (err) console.error(err.stack);
  });

export const setFee = (fee) => ({ type: 'set fee', fee });

const fetchFee = () => (dispatch, getState) => {
  const { contract } = getState();
  if (!contract) return;
  contract.instance.fee.call()
  .then((fee) => dispatch(setFee(fee)))
  .catch((err) => {
    console.error('could not fetch fee');
    if (err) console.error(err.stack);
  });
};

export const setOwner = (owner) => ({ type: 'set owner', owner });

export const fetchOwner = () => (dispatch, getState) => {
  const { contract } = getState();
  if (!contract) return;
  contract.instance.owner.call()
  .then((owner) => dispatch(setOwner(owner)))
  .catch((err) => {
    console.error('could not fetch owner');
    if (err) console.error(err.stack);
  });
};
