import { invoke } from "@tauri-apps/api/core";
import { Session, User } from "@supabase/supabase-js";

interface AuthSession {
  access_token: string;
  refresh_token?: string;
  expires_at?: string;
  user_id: string;
  user_email: string;
  stored_at: number;
}

interface AuthValidation {
  is_valid: boolean;
  is_expired: boolean;
  is_near_expiry: boolean;
  expires_in_seconds?: number;
  reason?: string;
}

export class AuthManager {
  private readonly SESSION_DURATION = 7 * 24 * 60 * 60 * 1000; // 7 days in milliseconds

  constructor() {
    // No need to initialize Store, using Tauri commands directly
  }

  /**
   * Convert Supabase Session to AuthSession format
   */
  private sessionToAuthSession(session: Session): AuthSession {
    return {
      access_token: session.access_token,
      refresh_token: session.refresh_token,
      expires_at: session.expires_at
        ? (new Date(session.expires_at).getTime() / 1000).toString()
        : undefined,
      user_id: session.user.id,
      user_email: session.user.email || "",
      stored_at: Math.floor(Date.now() / 1000), // Unix timestamp
    };
  }

  /**
   * Convert AuthSession back to Supabase Session format
   */
  private authSessionToSession(authSession: AuthSession): Session {
    const expiresAt = authSession.expires_at
      ? new Date(parseInt(authSession.expires_at) * 1000).toISOString()
      : undefined;

    return {
      access_token: authSession.access_token,
      refresh_token: authSession.refresh_token || "",
      expires_at: expiresAt,
      expires_in: authSession.expires_at
        ? parseInt(authSession.expires_at) - Math.floor(Date.now() / 1000)
        : 3600,
      token_type: "bearer",
      user: {
        id: authSession.user_id,
        email: authSession.user_email,
        aud: "authenticated",
        role: "authenticated",
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        app_metadata: {},
        user_metadata: {},
      },
    } as Session;
  }

  /**
   * Save authentication data securely using Tauri command
   */
  async saveAuth(session: Session): Promise<void> {
    try {
      const authSession = this.sessionToAuthSession(session);
      await invoke("save_auth_session", { session: authSession });
    } catch (error) {
      throw new Error("Failed to save authentication data");
    }
  }

  /**
   * Load authentication data from secure storage using Tauri command
   */
  async loadAuth(): Promise<Session | null> {
    try {
      const authSession = await invoke<AuthSession>("get_auth_session");

      return this.authSessionToSession(authSession);
    } catch (error) {
      return null;
    }
  }

  /**
   * Validate authentication using Tauri command
   */
  async validateAuth(): Promise<AuthValidation> {
    try {
      const validation = await invoke<AuthValidation>("validate_auth_session");
      return validation;
    } catch (error) {
      return {
        is_valid: false,
        is_expired: false,
        is_near_expiry: false,
        reason: "Failed to validate auth session",
      };
    }
  }

  /**
   * Clear authentication data from storage using Tauri command
   */
  async clearAuth(): Promise<void> {
    try {
      await invoke("clear_auth_session");
    } catch (error) {
      // Silent error handling
    }
  }

  /**
   * Check if stored session is about to expire (within 5 minutes)
   */
  isSessionNearExpiry(session: Session): boolean {
    if (!session.expires_at) return false;

    const expiresAt = new Date(session.expires_at).getTime();
    const fiveMinutes = 5 * 60 * 1000;

    return expiresAt - Date.now() < fiveMinutes;
  }

  /**
   * Get session expiry time as a readable string
   */
  getSessionExpiryTime(session: Session): string | null {
    if (!session.expires_at) return null;
    return new Date(session.expires_at).toLocaleString();
  }

  /**
   * Validate if a session object has required properties
   */
  isValidSession(session: any): session is Session {
    return (
      session &&
      typeof session === "object" &&
      session.access_token &&
      session.user &&
      session.user.id
    );
  }

  /**
   * Get authentication state for debugging
   */
  async getAuthDebugInfo(): Promise<{
    hasStoredAuth: boolean;
    sessionAge?: number;
    expiresAt?: string;
    isValid?: boolean;
    validation?: AuthValidation;
  }> {
    try {
      const validation = await this.validateAuth();
      const authSession = await invoke<AuthSession>("get_auth_session");

      const age = Date.now() / 1000 - authSession.stored_at;
      const expiresAt = authSession.expires_at
        ? new Date(parseInt(authSession.expires_at) * 1000).toISOString()
        : undefined;

      return {
        hasStoredAuth: true,
        sessionAge: age * 1000, // Convert back to milliseconds for consistency
        expiresAt,
        isValid: validation.is_valid,
        validation,
      };
    } catch (error) {
      return { hasStoredAuth: false };
    }
  }
}

// Export a singleton instance
export const authManager = new AuthManager();
