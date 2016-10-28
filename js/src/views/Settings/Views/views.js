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
import { connect } from 'react-redux';
import { bindActionCreators } from 'redux';
import { Checkbox } from 'material-ui';

import { Container, ContainerTitle } from '../../../ui';

import { toggleView } from '../actions';

import layout from '../layout.css';
import styles from './views.css';

class Views extends Component {
  static propTypes = {
    settings: PropTypes.object.isRequired,
    toggleView: PropTypes.func.isRequired
  }

  render () {
    return (
      <Container>
        <ContainerTitle title='Views' />
        <div className={ layout.layout }>
          <div className={ layout.overview }>
            <div>Manage the available application views, using only the parts of the application that is applicable to you.</div>
            <div>Are you an end-user? The defaults are setups for both beginner and advanced users alike.</div>
            <div>Are you a developer? Add some features to manage contracts are interact with application develoyments.</div>
            <div>Are you a miner or run a large-scale node? Add the features to give you all the information needed to watch the node operation.</div>
          </div>
          <div className={ layout.details }>
            { this.renderViews() }
          </div>
        </div>
      </Container>
    );
  }

  renderViews () {
    const { settings, toggleView } = this.props;

    return Object.keys(settings.views).map((id) => {
      const toggle = () => toggleView(id);
      const view = settings.views[id];
      const label = (
        <div className={ styles.header }>
          <div className={ styles.labelicon }>
            { view.icon }
          </div>
          <div className={ styles.label }>
            { view.label }
          </div>
        </div>
      );

      return (
        <div className={ styles.view } key={ id }>
          <Checkbox
            disabled={ view.fixed }
            label={ label }
            onCheck={ toggle }
            checked={ view.active }
            value={ view.active } />
          <div className={ styles.info }>
            { view.description }
          </div>
        </div>
      );
    });
  }
}

function mapStateToProps (state) {
  const { settings } = state;

  return { settings };
}

function mapDispatchToProps (dispatch) {
  return bindActionCreators({ toggleView }, dispatch);
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Views);
