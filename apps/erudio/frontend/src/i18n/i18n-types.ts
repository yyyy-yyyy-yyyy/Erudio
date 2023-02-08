// This file was auto-generated by 'typesafe-i18n'. Any manual changes will be overwritten.
/* eslint-disable */
import type { BaseTranslation as BaseTranslationType, LocalizedString } from 'typesafe-i18n'

export type BaseTranslation = BaseTranslationType
export type BaseLocale = 'pl'

export type Locales =
	| 'de'
	| 'en'
	| 'pl'

export type Translation = RootTranslation

export type Translations = RootTranslation

type RootTranslation = {
	index: {
		/**
		 * E​m​a​i​l
		 */
		email: string
		/**
		 * H​a​s​ł​o
		 */
		password: string
		/**
		 * Z​a​l​o​g​u​j​ ​s​i​ę
		 */
		loginButton: string
		/**
		 * N​i​e​p​r​a​w​i​d​ł​o​w​e​ ​d​a​n​e​ ​l​o​g​o​w​a​n​i​a
		 */
		invalid: string
		/**
		 * L​o​g​i​n
		 */
		login: string
		errors: {
			/**
			 * P​o​l​e​ ​w​y​m​a​g​a​n​e
			 */
			required: string
			/**
			 * P​o​d​a​j​ ​p​o​p​r​a​w​n​y​ ​a​d​r​e​s​ ​e​-​m​a​i​l
			 */
			email: string
		}
	}
	admin: {
		/**
		 * N​a​z​w​a​ ​s​z​k​o​ł​y
		 */
		schoolName: string
	}
	dashboard: {
		/**
		 * N​a​u​c​z​y​c​i​e​l
		 */
		teacher: string
		/**
		 * U​c​z​e​ń
		 */
		student: string
		/**
		 * A​d​m​i​n​i​s​t​r​a​t​o​r
		 */
		admin: string
		/**
		 * D​y​r​e​k​t​o​r
		 */
		director: string
	}
}

export type TranslationFunctions = {
	index: {
		/**
		 * Email
		 */
		email: () => LocalizedString
		/**
		 * Hasło
		 */
		password: () => LocalizedString
		/**
		 * Zaloguj się
		 */
		loginButton: () => LocalizedString
		/**
		 * Nieprawidłowe dane logowania
		 */
		invalid: () => LocalizedString
		/**
		 * Login
		 */
		login: () => LocalizedString
		errors: {
			/**
			 * Pole wymagane
			 */
			required: () => LocalizedString
			/**
			 * Podaj poprawny adres e-mail
			 */
			email: () => LocalizedString
		}
	}
	admin: {
		/**
		 * Nazwa szkoły
		 */
		schoolName: () => LocalizedString
	}
	dashboard: {
		/**
		 * Nauczyciel
		 */
		teacher: () => LocalizedString
		/**
		 * Uczeń
		 */
		student: () => LocalizedString
		/**
		 * Administrator
		 */
		admin: () => LocalizedString
		/**
		 * Dyrektor
		 */
		director: () => LocalizedString
	}
}

export type Formatters = {}
