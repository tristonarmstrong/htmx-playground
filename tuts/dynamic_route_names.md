# Dynamic route names

```tsx
/** All application routes. Assign new routes here as needed*/
export const ROUTES = {
  MY_CASES: {
    main: 'mycases/',
    sub: {
      // --- nothing here yet
    }
  },
  NEW_CASES: {
    main: 'new-cases/',
    sub: {
      VIEW_FINDER: 'new-cases/view-finder',
      APPROVE_IMAGE: 'new-cases/view-finder/approve-image',
      CLASSIFY_IMAGE: 'new-cases/view-finder/classify-image',
      CAPTURED_LIST: 'new-cases/captured-list'
    }
  },
  LOGIN: {
    main: 'login/',
    sub: {
      MFA: 'login/mfa',
      MFA_PHONE: 'login/mfa-phone',
      FORGOT_PASSWORD: 'login/forgot-password'
    }
  },
  PROFILE: {
    main: 'profile/', // sometimes we dont want to actually go to this route. so we'll skip this one
    sub: {
      PASSWORD: 'profile/password',
      PHONE: 'profile/phone'
    }
  },
  EXTERNAL: {
    main: 'https://ventrahealth.com',
    sub: {
      // put any other routes here - may not have any
    }
  }
} as const

// Dont mess with these, these are builders for the values below
type KeyExtractor<T> = keyof T
type RoutesType = typeof ROUTES
type RoutesKeys = KeyExtractor<RoutesType>
type RoutesSubsKeys = KeyExtractor<Union.Merge<RoutesType[RoutesKeys]['sub']>>

/** All main routes available to you */
export type MainRoutes = RoutesType[RoutesKeys]['main']
/** All sub routes available to you */
export type SubRoutes = Union.Merge<RoutesType[RoutesKeys]['sub']>[RoutesSubsKeys]
```