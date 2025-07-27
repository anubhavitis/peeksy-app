import {
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
} from "react";
import { User, Session } from "@supabase/supabase-js";
import { supabase } from "../lib/supabase";
import { authManager } from "../lib/authManager";

interface AuthContextType {
  user: User | null;
  session: Session | null;
  loading: boolean;
  signIn: (email: string, password: string) => Promise<{ error: any }>;
  signUp: (email: string, password: string) => Promise<{ error: any }>;
  signOut: () => Promise<void>;
  refreshSession: () => Promise<boolean>;
  isSessionNearExpiry: () => boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider = ({ children }: AuthProviderProps) => {
  const [user, setUser] = useState<User | null>(null);
  const [session, setSession] = useState<Session | null>(null);
  const [loading, setLoading] = useState(true);

  // Initialize auth state
  useEffect(() => {
    let mounted = true;

    const initializeAuth = async () => {
      try {
        // First, try to load from secure storage
        const storedSession = await authManager.loadAuth();

        if (storedSession && authManager.isValidSession(storedSession)) {
          // Set the session in Supabase client
          await supabase.auth.setSession({
            access_token: storedSession.access_token,
            refresh_token: storedSession.refresh_token || "",
          });

          if (mounted) {
            setSession(storedSession);
            setUser(storedSession.user);
          }
        } else {
          // Fallback to getting current session from Supabase
          const {
            data: { session: currentSession },
          } = await supabase.auth.getSession();

          if (currentSession && mounted) {
            setSession(currentSession);
            setUser(currentSession.user);
            // Save to secure storage for future use
            await authManager.saveAuth(currentSession);
          }
        }
      } catch (error) {
        // Silent error handling for initialization
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    };

    initializeAuth();

    // Listen for auth changes from Supabase
    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange(async (event, session) => {
      if (mounted) {
        setSession(session);
        setUser(session?.user ?? null);
        setLoading(false);
      }

      // Handle auth events
      if (event === "SIGNED_IN" && session) {
        // Save new session to secure storage
        await authManager.saveAuth(session);
      } else if (event === "SIGNED_OUT") {
        // Clear secure storage
        await authManager.clearAuth();
      } else if (event === "TOKEN_REFRESHED" && session) {
        // Update stored session
        await authManager.saveAuth(session);
      }
    });

    return () => {
      mounted = false;
      subscription.unsubscribe();
    };
  }, []);

  // Auto-refresh session when it's near expiry
  useEffect(() => {
    if (!session) return;

    const checkAndRefreshSession = async () => {
      if (authManager.isSessionNearExpiry(session)) {
        await refreshSession();
      }
    };

    // Check every minute for session expiry
    const interval = setInterval(checkAndRefreshSession, 60000);

    return () => clearInterval(interval);
  }, [session]);

  const signIn = async (email: string, password: string) => {
    setLoading(true);
    try {
      const { data, error } = await supabase.auth.signInWithPassword({
        email,
        password,
      });

      if (data.session) {
        // Session will be saved automatically via onAuthStateChange
        setUser(data.user);
        setSession(data.session);
      }

      return { error };
    } catch (err: any) {
      return { error: err };
    } finally {
      setLoading(false);
    }
  };

  const signUp = async (email: string, password: string) => {
    setLoading(true);
    try {
      const { data, error } = await supabase.auth.signUp({
        email,
        password,
      });

      if (data.session) {
        // Session will be saved automatically via onAuthStateChange
        setUser(data.user);
        setSession(data.session);
      }

      return { error };
    } catch (err: any) {
      return { error: err };
    } finally {
      setLoading(false);
    }
  };

  const signOut = async () => {
    setLoading(true);
    try {
      // Clear secure storage first
      await authManager.clearAuth();

      // Then sign out from Supabase
      await supabase.auth.signOut();

      setUser(null);
      setSession(null);
    } catch (error) {
      // Silent error handling for sign out
    } finally {
      setLoading(false);
    }
  };

  const refreshSession = async (): Promise<boolean> => {
    try {
      const { data, error } = await supabase.auth.refreshSession();

      if (error) {
        return false;
      }

      if (data.session) {
        setSession(data.session);
        setUser(data.session.user);
        // Session will be saved automatically via onAuthStateChange
        return true;
      }

      return false;
    } catch (error) {
      return false;
    }
  };

  const isSessionNearExpiry = (): boolean => {
    if (!session) return false;
    return authManager.isSessionNearExpiry(session);
  };

  const value = {
    user,
    session,
    loading,
    signIn,
    signUp,
    signOut,
    refreshSession,
    isSessionNearExpiry,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
