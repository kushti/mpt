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

import { Form, Input } from '../../../ui';

import styles from '../createAccount.css';

import { ERRORS } from '../NewAccount';

export default class RecoveryPhrase extends Component {
  static propTypes = {
    onChange: PropTypes.func.isRequired
  }

  state = {
    recoveryPhrase: '',
    recoveryPhraseError: ERRORS.noPhrase,
    accountName: '',
    accountNameError: ERRORS.noName,
    passwordHint: '',
    password1: '',
    password1Error: ERRORS.invalidPassword,
    password2: '',
    password2Error: ERRORS.noMatchPassword,
    isValidPass: false,
    isValidName: false,
    isValidPhrase: false
  }

  componentWillMount () {
    this.props.onChange(false, {});
  }

  render () {
    const { accountName, accountNameError, passwordHint, password1, password1Error, password2, password2Error, recoveryPhrase } = this.state;

    return (
      <Form>
        <Input
          hint='the account recovery phrase'
          label='account recovery phrase'
          multiLine
          rows={ 1 }
          value={ recoveryPhrase }
          onChange={ this.onEditPhrase } />
        <Input
          label='account name'
          hint='a descriptive name for the account'
          error={ accountNameError }
          value={ accountName }
          onChange={ this.onEditAccountName } />
        <Input
          label='password hint'
          hint='(optional) a hint to help with remembering the password'
          value={ passwordHint }
          onChange={ this.onEditPasswordHint } />
        <div className={ styles.passwords }>
          <div className={ styles.password }>
            <Input
              className={ styles.password }
              label='password'
              hint='a strong, unique password'
              type='password'
              error={ password1Error }
              value={ password1 }
              onChange={ this.onEditPassword1 } />
          </div>
          <div className={ styles.password }>
            <Input
              className={ styles.password }
              label='password (repeat)'
              hint='verify your password'
              type='password'
              error={ password2Error }
              value={ password2 }
              onChange={ this.onEditPassword2 } />
          </div>
        </div>
      </Form>
    );
  }

  updateParent = () => {
    const { isValidName, isValidPass, isValidPhrase, accountName, passwordHint, password1, recoveryPhrase } = this.state;
    const isValid = isValidName && isValidPass && isValidPhrase;

    this.props.onChange(isValid, {
      name: accountName,
      passwordHint,
      password: password1,
      phrase: recoveryPhrase
    });
  }

  onEditPasswordHint = (event, value) => {
    this.setState({
      passwordHint: value
    });
  }

  onEditPhrase = (event) => {
    const value = event.target.value;
    let error = null;

    if (!value || value.trim().length < 25) {
      error = ERRORS.noPhrase;
    }

    this.setState({
      recoveryPhrase: value,
      recoveryPhraseError: error,
      isValidPhrase: !error
    }, this.updateParent);
  }

  onEditAccountName = (event) => {
    const value = event.target.value;
    let error = null;

    if (!value || value.trim().length < 2) {
      error = ERRORS.noName;
    }

    this.setState({
      accountName: value,
      accountNameError: error,
      isValidName: !error
    }, this.updateParent);
  }

  onEditPassword1 = (event) => {
    const value = event.target.value;
    let error1 = null;
    let error2 = null;

    if (!value || value.trim().length < 8) {
      error1 = ERRORS.invalidPassword;
    }

    if (value !== this.state.password2) {
      error2 = ERRORS.noMatchPassword;
    }

    this.setState({
      password1: value,
      password1Error: error1,
      password2Error: error2,
      isValidPass: !error1 && !error2
    }, this.updateParent);
  }

  onEditPassword2 = (event) => {
    const value = event.target.value;
    let error2 = null;

    if (value !== this.state.password1) {
      error2 = ERRORS.noMatchPassword;
    }

    this.setState({
      password2: value,
      password2Error: error2,
      isValidPass: !error2
    }, this.updateParent);
  }
}
