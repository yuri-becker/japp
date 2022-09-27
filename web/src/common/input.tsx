/*
 * Copyright (C) 2022 - This file is part of "JAPP".
 *
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with JAPP.  If not, see http://www.gnu.org/licenses/.
 */

import classNames from 'classnames'
import { ChangeEventHandler, createRef, FC, MouseEventHandler, ReactNode, useState } from 'react'
import './input.css'

interface InputProps {
  value?: string,
  change?: (value?: string) => void,
  hint?: ReactNode,
  icon?: ReactNode,
  label: ReactNode,
  name: string,
  errorMessage?: string
}

export const Input: FC<InputProps> = props => {
  const input = createRef<HTMLInputElement>()
  const [touched, setTouched] = useState<boolean>(false)
  const onChange: ChangeEventHandler<HTMLInputElement> = event => {
    if (props.change) props.change(event.target.value)
    setTouched(true)
  }
  const click: MouseEventHandler<HTMLDivElement> = () => input.current?.focus()
  const onKeyDown = () => setTouched(true)

  return <div className={classNames('input', { 'input--has-error': props.errorMessage && touched })}>
    <div className="input__container" onClick={click}>
      {props.icon && <div className="input__icon" aria-hidden={true}>{props.icon}</div>}
      <label htmlFor={props.name} className="input__label">{props.label}</label>
      <input name={props.name} id={props.name} value={props.value} onChange={onChange} onKeyDown={onKeyDown}
             ref={input}/>
    </div>
    <div className="input__hint">
      {props.errorMessage && touched && <span className="input__error">{props.errorMessage}<br/></span>}
      {props.hint}
    </div>
  </div>
}
