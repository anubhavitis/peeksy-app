"use client";
import { useEffect, useState, useCallback } from "react";
import { useAuth } from "../contexts/AuthContext";
import { supabase } from "../lib/supabase";

// Types
interface Subscription {
  dodo_data: {
    status: string;
  };
}

interface UserProfile {
  subscriptions: Subscription[];
}

interface UserProfileProps {
  className?: string;
}

const UserProfile: React.FC<UserProfileProps> = ({ className = "" }) => {
  const { user, session, signOut, loading } = useAuth();
  const [userProfile, setUserProfile] = useState<UserProfile | null>(null);
  const [profileLoading, setProfileLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Get user profile from Supabase Edge Function
  const getUserProfile = useCallback(async () => {
    if (!session?.access_token) {
      setError("No access token available");
      return;
    }

    setProfileLoading(true);
    setError(null);

    try {
      const headers = {
        Authorization: `Bearer ${session.access_token}`,
      };

      const response = await supabase.functions.invoke("get-user-details", {
        headers,
      });

      if (response.error) {
        setError(`Failed to fetch profile: ${response.error.message}`);
        return;
      }

      setUserProfile(response.data);
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error occurred";
      setError(`Error fetching profile: ${errorMessage}`);
    } finally {
      setProfileLoading(false);
    }
  }, [session?.access_token]);

  useEffect(() => {
    if (user) {
      getUserProfile();
    }
  }, [user, getUserProfile]);

  const handleSignOut = useCallback(async () => {
    try {
      await signOut();
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error occurred";
      console.error("Error signing out:", errorMessage);
    }
  }, [signOut]);

  const getSubscriptionStatus = (): string => {
    if (!userProfile?.subscriptions?.[0]?.dodo_data?.status) {
      return "Peeksy Lite";
    }

    return userProfile.subscriptions[0].dodo_data.status === "active"
      ? "Peeksy Pro"
      : "Peeksy Lite";
  };

  const getInitials = (email: string): string => {
    return email.charAt(0).toUpperCase();
  };

  // Loading state
  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  // No user state
  if (!user) {
    return null;
  }

  return (
    <div
      className={`w-full mx-auto px-6 sm:px-10 flex flex-col gap-6 ${className}`}
    >
      {/* Header Section */}
      <div className="text-center">
        <div className="w-20 h-20 bg-indigo-100 dark:bg-indigo-900 rounded-full flex items-center justify-center mx-auto mb-4 shadow-lg">
          <span className="text-2xl font-bold text-indigo-600 dark:text-indigo-400">
            {getInitials(user.email || "")}
          </span>
        </div>
        <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">
          Welcome back!
        </h2>
        <p className="text-gray-600 dark:text-gray-400 mt-1">{user.email}</p>
      </div>

      {/* Profile Loading State */}
      {profileLoading && (
        <div className="flex items-center justify-center py-4">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-indigo-600"></div>
          <span className="ml-2 text-gray-600 dark:text-gray-400">
            Loading profile...
          </span>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md p-4">
          <p className="text-red-800 dark:text-red-200 text-sm">{error}</p>
          <button
            onClick={getUserProfile}
            className="mt-2 text-red-700 dark:text-red-300 text-sm underline hover:no-underline"
          >
            Try again
          </button>
        </div>
      )}

      {/* Subscription Status */}
      {!profileLoading && !error && userProfile && (
        <div className="flex flex-col gap-2 justify-center items-center">
          <div className="bg-gray-50 dark:bg-gray-800 rounded-lg px-4 py-3">
            <span className="font-medium text-gray-900 dark:text-gray-100">
              Status:{" "}
              <span className="text-indigo-600 dark:text-indigo-400">
                {getSubscriptionStatus()}
              </span>
            </span>
          </div>
        </div>
      )}

      {/* Sign Out Button */}
      <button
        onClick={handleSignOut}
        className="w-full mt-6 py-3 px-4 bg-red-600 hover:bg-red-700 text-white 
                 font-medium rounded-md transition-all duration-200
                 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2
                 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={profileLoading}
      >
        Sign Out
      </button>
    </div>
  );
};

export default UserProfile;
