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

const footer = document.getElementsByTagName('footer')[0]
const links = [
  ['Imprint', '/imprint'],
  ['Data Privacy', '/data-privacy'],
  ['Icons and Illustrations by icons8', 'https://icons8.com', true]
]
links.forEach(link => {
  const elem = document.createElement('a')
  elem.innerText = link[0]
  elem.href = link[1]
  if(link[2]) {
    elem.target = '_blank'
  }
  footer.appendChild(elem)
})
