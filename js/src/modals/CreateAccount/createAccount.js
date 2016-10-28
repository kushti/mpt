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

import React, { Component, PropTypes } from 'react';
import ActionDone from 'material-ui/svg-icons/action/done';
import ActionDoneAll from 'material-ui/svg-icons/action/done-all';
import ContentClear from 'material-ui/svg-icons/content/clear';
import NavigationArrowBack from 'material-ui/svg-icons/navigation/arrow-back';
import NavigationArrowForward from 'material-ui/svg-icons/navigation/arrow-forward';

import { Button, Modal } from '../../ui';

import AccountDetails from './AccountDetails';
import AccountDetailsGeth from './AccountDetailsGeth';
import CreationType from './CreationType';
import NewAccount from './NewAccount';
import NewGeth from './NewGeth';
import NewImport from './NewImport';
import RecoveryPhrase from './RecoveryPhrase';

const TITLES = {
  type: 'creation type',
  create: 'create account',
  info: 'account information',
  import: 'import wallet'
};
const STAGE_NAMES = [TITLES.type, TITLES.create, TITLES.info];
const STAGE_IMPORT = [TITLES.type, TITLES.import, TITLES.info];

export default class CreateAccount extends Component {
  static contextTypes = {
    api: PropTypes.object.isRequired,
    store: PropTypes.object.isRequired
  }

  static propTypes = {
    accounts: PropTypes.object.isRequired,
    onClose: PropTypes.func,
    onUpdate: PropTypes.func
  }

  state = {
    address: null,
    name: null,
    passwordHint: null,
    password: null,
    phrase: null,
    json: null,
    canCreate: false,
    createType: null,
    gethAddresses: [],
    stage: 0
  }

  render () {
    const { createType, stage } = this.state;
    const steps = createType === 'fromNew'
      ? STAGE_NAMES
      : STAGE_IMPORT;

    return (
      <Modal
        visible
        actions={ this.renderDialogActions() }
        current={ stage }
        steps={ steps }>
        { this.renderPage() }
      </Modal>
    );
  }

  renderPage () {
    const { createType, stage } = this.state;
    const { accounts } = this.props;

    switch (stage) {
      case 0:
        return (
          <CreationType
            onChange={ this.onChangeType } />
        );

      case 1:
        if (createType === 'fromNew') {
          return (
            <NewAccount
              onChange={ this.onChangeDetails } />
          );
        } else if (createType === 'fromGeth') {
          return (
            <NewGeth
              accounts={ accounts }
              onChange={ this.onChangeGeth } />
          );
        } else if (createType === 'fromPhrase') {
          return (
            <RecoveryPhrase
              onChange={ this.onChangeDetails } />
          );
        }

        return (
          <NewImport
            onChange={ this.onChangeWallet } />
        );

      case 2:
        if (createType === 'fromGeth') {
          return (
            <AccountDetailsGeth
              addresses={ this.state.gethAddresses } />
          );
        }

        return (
          <AccountDetails
            address={ this.state.address }
            name={ this.state.name }
            phrase={ this.state.phrase } />
        );
    }
  }

  renderDialogActions () {
    const { createType, stage } = this.state;

    switch (stage) {
      case 0:
        return [
          <Button
            icon={ <ContentClear /> }
            label='Cancel'
            onClick={ this.onClose } />,
          <Button
            icon={ <NavigationArrowForward /> }
            label='Next'
            onClick={ this.onNext } />
        ];
      case 1:
        const createLabel = createType === 'fromNew'
          ? 'Create'
          : 'Import';

        return [
          <Button
            icon={ <ContentClear /> }
            label='Cancel'
            onClick={ this.onClose } />,
          <Button
            icon={ <NavigationArrowBack /> }
            label='Back'
            onClick={ this.onPrev } />,
          <Button
            icon={ <ActionDone /> }
            label={ createLabel }
            disabled={ !this.state.canCreate }
            onClick={ this.onCreate } />
        ];

      case 2:
        return (
          <Button
            icon={ <ActionDoneAll /> }
            label='Close'
            onClick={ this.onClose } />
        );
    }
  }

  onNext = () => {
    this.setState({
      stage: this.state.stage + 1
    });
  }

  onPrev = () => {
    this.setState({
      stage: this.state.stage - 1
    });
  }

  onCreate = () => {
    const { createType } = this.state;
    const { api } = this.context;

    this.setState({
      canCreate: false
    });

    if (createType === 'fromNew' || createType === 'fromPhrase') {
      return api.personal
        .newAccountFromPhrase(this.state.phrase, this.state.password)
        .then((address) => {
          this.setState({ address });
          return api.personal
            .setAccountName(address, this.state.name)
            .then(() => api.personal.setAccountMeta(address, { passwordHint: this.state.passwordHint }));
        })
        .then(() => {
          this.onNext();
          this.props.onUpdate && this.props.onUpdate();
        })
        .catch((error) => {
          console.error('onCreate', error);

          this.setState({
            canCreate: true
          });

          this.newError(error);
        });
    } else if (createType === 'fromGeth') {
      return api.personal
        .importGethAccounts(this.state.gethAddresses)
        .then((result) => {
          console.log('result', result);

          return Promise.all(this.state.gethAddresses.map((address) => {
            return api.personal.setAccountName(address, 'Geth Import');
          }));
        })
        .then(() => {
          this.onNext();
          this.props.onUpdate && this.props.onUpdate();
        })
        .catch((error) => {
          console.error('onCreate', error);

          this.setState({
            canCreate: true
          });

          this.newError(error);
        });
    }

    return api.personal
      .newAccountFromWallet(this.state.json, this.state.password)
      .then((address) => {
        this.setState({
          address: address
        });

        return api.personal
          .setAccountName(address, this.state.name)
          .then(() => api.personal.setAccountMeta(address, { passwordHint: this.state.passwordHint }));
      })
      .then(() => {
        this.onNext();
        this.props.onUpdate && this.props.onUpdate();
      })
      .catch((error) => {
        console.error('onCreate', error);

        this.setState({
          canCreate: true
        });

        this.newError(error);
      });
  }

  onClose = () => {
    this.setState({
      stage: 0,
      canCreate: false
    }, () => {
      this.props.onClose && this.props.onClose();
    });
  }

  onChangeType = (value) => {
    this.setState({
      createType: value
    });
  }

  onChangeDetails = (valid, { name, passwordHint, address, password, phrase }) => {
    this.setState({
      canCreate: valid,
      name,
      passwordHint,
      address,
      password,
      phrase
    });
  }

  onChangeGeth = (valid, gethAddresses) => {
    this.setState({
      canCreate: valid,
      gethAddresses
    });
  }

  onChangeWallet = (valid, { name, passwordHint, password, json }) => {
    this.setState({
      canCreate: valid,
      name,
      passwordHint,
      password,
      json
    });
  }

  newError = (error) => {
    const { store } = this.context;

    store.dispatch({ type: 'newError', error });
  }
}
