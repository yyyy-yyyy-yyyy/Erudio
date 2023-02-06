// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "public.version", input: never, result: string } | 
        { key: "super_admin.version", input: never, result: string } | 
        { key: "user.me", input: never, result: UserMeResponse },
    mutations: 
        { key: "file.upload", input: UploadRequest, result: UploadResponse } | 
        { key: "public.login", input: LoginRequest, result: LoginResponse } | 
        { key: "public.login.admin", input: AdminLoginRequest, result: AdminLoginResponse } | 
        { key: "public.register", input: RegisterRequest, result: null } | 
        { key: "user.logout", input: never, result: null },
    subscriptions: never
};

export interface AdminLoginRequest { login: string, password: string }

export type AdminLoginResponse = { t: "Success" } | { t: "TwoFactorAuth", c: TwoFactorAuthType }

export interface LoginRequest { email: string, password: string }

export type LoginResponse = { t: "Success" } | { t: "TwoFactorAuth", c: TwoFactorAuthType }

export interface RegisterRequest { idempotence_token: string, email: string, password: string, first_name: string, middle_name: string | null, last_name: string, code: string | null }

export type SchoolRelationType = "student" | "teacher" | "admin" | "director"

export type TwoFactorAuthType = "GoogleAuth" | "Sms" | "EMail"

export interface UploadRequest { idempotence_token: string, idk: string }

export interface UploadResponse { presigned_url: string }

export interface UserMeResponse { id: string, display_name: string, school_relations: Array<[SchoolRelationType, string]> }
