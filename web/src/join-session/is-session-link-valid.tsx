/*
 * Copyright (C) 2023 - This file is part of "JAPP".
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 * terms of version 3 of the GNU Affero General Public License as published by the
 * Free Software Foundation.
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 * details.
 * You should have received a copy of the GNU Affero General Public License
 * long with JAPP.  If not, see http://www.gnu.org/licenses/.
 */

const sessionLinkRegex = /^\/?app\/session\/[\d\w-]+\/[\d\w-]+\/?$/

export const isSessionLinkValid = (link?: string) => {
  if (!link) {
    return false
  }
  const beginsWithOrigin = link.startsWith(document.location.origin)
  if (!beginsWithOrigin) {
    return false
  }
  return sessionLinkRegex.test(link.slice(document.location.origin.length))
}
