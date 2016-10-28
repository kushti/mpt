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

import * as abis from './abi';

export default class Registry {
  constructor (api) {
    this._api = api;
    this._contracts = [];
    this._instance = null;

    this.getInstance();
  }

  getInstance () {
    return new Promise((resolve, reject) => {
      if (this._instance) {
        resolve(this._instance);
        return;
      }

      this._api.ethcore
        .registryAddress()
        .then((address) => {
          this._instance = this._api.newContract(abis.registry, address).instance;
          resolve(this._instance);
        })
        .catch(reject);
    });
  }

  getContractInstance (_name) {
    const name = _name.toLowerCase();

    return new Promise((resolve, reject) => {
      if (this._contracts[name]) {
        resolve(this._contracts[name]);
        return;
      }

      this
        .lookupAddress(name)
        .then((address) => {
          this._contracts[name] = this._api.newContract(abis[name], address).instance;
          resolve(this._contracts[name]);
        })
        .catch(reject);
    });
  }

  lookupAddress (_name) {
    const name = _name.toLowerCase();
    const sha3 = this._api.util.sha3(name);

    return this.getInstance().then((instance) => {
      return instance.getAddress.call({}, [sha3, 'A']);
    })
    .then((address) => {
      console.log('lookupAddress', name, sha3, address);
      return address;
    });
  }
}
