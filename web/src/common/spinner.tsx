/*
 * Copyright (C) 2022 - This file is part of "japp".
 *
 * "japp" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "japp" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with japp.  If not, see http://www.gnu.org/licenses/.
 */

import { FC } from 'react'
import classNames from 'classnames'
import './spinner.css'

export const Spinner: FC<{ size: 'inline' | 'page', color?: 'primary' }> = ({ size, color }) => <div
  className={classNames(
    'spinner',
    {
      'spinner--inline': size === 'inline',
      'spinner--page': size === 'page',
      'spinner--color-primary': color === 'primary'
    }
  )}>
  <i className="icons8">&#xf107;</i>
</div>
