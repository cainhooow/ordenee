export type Person = {
    id: number,
    name: string,
    email?: string,
    person_id?: string,
    tel_num?: number,
    is_technical: boolean,
    created_at: string,
    updated_at?: string
}

export type PersonAddress = {
    id: number,
    address: string,
    home_num?: number,
    street?: string,
    city?: string,
    person_id: number
}

export type ReturnablePerson = {
    person: Person,
    addresses: PersonAddress[]
}