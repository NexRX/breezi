import * as v from 'valibot'

export const schema = v.object({ "email": v.pipe(v.string(), v.email()), "id": v.pipe(v.string(), v.regex(/^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$/)), "password": v.pipe(v.string(), v.minLength(5), v.maxLength(1024)), "username": v.pipe(v.string(), v.regex(/^[a-zA-Z0-9_]{1,32}$/)) })
