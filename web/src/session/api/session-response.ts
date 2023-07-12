import { ParticipantResponse } from './participant-response'

export interface SessionResponse {
  name: string,
  participants: ParticipantResponse[],
  scale: string[],
}
