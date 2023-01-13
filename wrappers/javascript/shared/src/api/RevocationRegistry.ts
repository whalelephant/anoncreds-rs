import type { RevocationRegistryDefinition } from './RevocationRegistryDefinition'

import { AnoncredsObject } from '../AnoncredsObject'
import { anoncreds } from '../register'

import { RevocationRegistryDelta } from './RevocationRegistryDelta'

export type RevokeCredentialOptions = {
  revocationRegistryDefinition: RevocationRegistryDefinition
  credentialRevocationIndex: number
  tailsPath: string
}

export type UpdateRevocationRegistryOptions = {
  revocationRegistryDefinition: RevocationRegistryDefinition
  issued: number[]
  revoked: number[]
  tailsDirectoryPath: string
}

export class RevocationRegistry extends AnoncredsObject {
  public static load(json: string) {
    return new RevocationRegistry(anoncreds.revocationRegistryFromJson({ json }).handle)
  }

  public revokeCredential(options: RevokeCredentialOptions) {
    const { revocationRegistry, revocationRegistryDelta } = anoncreds.revokeCredential({
      revocationRegistryDefinition: options.revocationRegistryDefinition.handle,
      revocationRegistry: this._handle,
      credentialRevocationIndex: options.credentialRevocationIndex,
      tailsPath: options.tailsPath,
    })

    this._handle = revocationRegistry

    return new RevocationRegistryDelta(revocationRegistryDelta.handle)
  }

  public update(options: UpdateRevocationRegistryOptions) {
    const { revocationRegistry, revocationRegistryDelta } = anoncreds.updateRevocationRegistry({
      revocationRegistryDefinition: options.revocationRegistryDefinition.handle,
      revocationRegistry: this._handle,
      issued: options.issued,
      revoked: options.revoked,
      tailsDirectoryPath: options.tailsDirectoryPath,
    })

    this._handle = revocationRegistry

    return new RevocationRegistryDelta(revocationRegistryDelta.handle)
  }
}
