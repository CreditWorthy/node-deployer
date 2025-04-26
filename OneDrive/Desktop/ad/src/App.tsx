import { useState, useEffect } from "react";
import { useMutation, useQuery } from "convex/react";
import { api } from "../convex/_generated/api";
import { toast } from "sonner";

const FAKE_TRANSACTIONS = [
  "HK2Zd...9qWx scheduled SOL → USDC swap",
  "7NPn8...mK3v set limit order at $23.4",
  "Aq5tR...vX2p scheduled USDT transfer",
  "J8kLm...4YwN private SPL token swap",
  "5fPxQ...2HgB timed token vesting",
];

const FAKE_STATS = [
  { label: "Total Volume", value: "$847K" },
  { label: "Users Waiting", value: "892" },
  { label: "Chains Supported", value: "3" },
];

const SERVER_STATS = [
  { 
    location: "Frankfurt",
    latency: "0.3ms",
    icon: "🇩🇪"
  },
  {
    location: "New York",
    latency: "0.4ms",
    icon: "🇺🇸"
  },
  {
    location: "Singapore",
    latency: "0.3ms",
    icon: "🇸🇬"
  },
  {
    location: "Tokyo",
    latency: "0.4ms",
    icon: "🇯🇵"
  }
];

export default function App() {
  const [email, setEmail] = useState("");
  const [currentTx, setCurrentTx] = useState(0);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [hasJoined, setHasJoined] = useState(() => {
    return localStorage.getItem("hasJoinedWaitlist") === "true";
  });
  
  const joinWaitlist = useMutation(api.waitlist.join);
  const waitlistCount = useQuery(api.waitlist.getCount) ?? 500;

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentTx(prev => (prev + 1) % FAKE_TRANSACTIONS.length);
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (hasJoined) {
      toast.error("You've already joined the waitlist!");
      return;
    }

    if (!email.match(/^[^\s@]+@[^\s@]+\.[^\s@]+$/)) {
      toast.error("Please enter a valid email address");
      return;
    }
    
    if (isSubmitting) return;
    
    setIsSubmitting(true);
    try {
      await joinWaitlist({ email });
      localStorage.setItem("hasJoinedWaitlist", "true");
      setHasJoined(true);
      toast.success("Welcome to the future of DeFi! 🚀", {
        description: "You're now on the waitlist. We'll keep you updated on our progress.",
        duration: 5000,
      });
      setEmail("");
    } catch (err: any) {
      if (err.message === "Email already registered") {
        toast.error("This email is already on the waitlist!");
      } else {
        toast.error("Failed to join waitlist. Please try again later.");
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      <div className="max-w-6xl mx-auto px-4 py-16">
        <main className="text-center">
          <div className="relative">
            <div className="absolute inset-0 blur-[100px] bg-gradient-to-r from-[#9945FF]/30 via-[#14F195]/30 to-[#00C2FF]/30 rounded-full" />
            <h1 className="relative text-6xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-[#9945FF] via-[#14F195] to-[#00C2FF] mb-6">
              TimedTX
            </h1>
          </div>
          
          <p className="text-2xl text-gray-400 mb-8">
            The Future of Cross-Chain Automation
          </p>

          <div className="bg-gray-800/50 backdrop-blur-sm rounded-xl p-4 mb-12 overflow-hidden relative h-[60px] flex items-center justify-center">
            <div className="animate-fade-in-up absolute inset-0 flex items-center justify-center">
              <div className="flex items-center gap-3">
                <div className="w-2 h-2 rounded-full bg-[#14F195] animate-pulse" />
                <p className="text-[#14F195] font-mono">
                  {FAKE_TRANSACTIONS[currentTx]}
                </p>
              </div>
            </div>
          </div>

          <div className="grid md:grid-cols-3 gap-8 mb-16">
            {FAKE_STATS.map((stat, i) => (
              <div key={i} className="bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50">
                <div className="text-2xl font-bold text-white mb-2">{stat.value}</div>
                <div className="text-gray-400">{stat.label}</div>
              </div>
            ))}
          </div>

          <div className="grid md:grid-cols-3 gap-8 mb-16">
            <Feature 
              title="Time-Delayed" 
              description="Schedule transactions to execute at specific times or after delays"
            />
            <Feature 
              title="Cross-Chain" 
              description="Execute transactions across multiple blockchains seamlessly"
            />
            <Feature 
              title="Privacy Options" 
              description="Choose between transparent or private transaction modes"
            />
          </div>

          <div className="bg-gray-800/50 backdrop-blur-sm rounded-2xl border border-gray-700/50 p-8 max-w-md mx-auto">
            <h2 className="text-2xl font-semibold mb-2">Join {waitlistCount.toLocaleString()} Others</h2>
            <p className="text-gray-400 mb-6">Get early access to the future of DeFi</p>
            {hasJoined ? (
              <div className="text-[#14F195] font-medium">
                You're on the waitlist! We'll keep you updated.
              </div>
            ) : (
              <form onSubmit={handleSubmit} className="flex gap-2">
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="Enter your email"
                  className="flex-1 px-4 py-2 rounded bg-gray-900/50 border border-gray-700/50 focus:outline-none focus:ring-2 focus:ring-[#9945FF] text-white placeholder-gray-500"
                  required
                  disabled={isSubmitting}
                />
                <button
                  type="submit"
                  disabled={isSubmitting}
                  className={`px-6 py-2 bg-gradient-to-r from-[#9945FF] to-[#00C2FF] text-white rounded hover:from-[#8935FF] hover:to-[#00B2FF] transition transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none`}
                >
                  Join
                </button>
              </form>
            )}
          </div>

          {/* Infrastructure Section */}
          <div className="mt-16 bg-gray-800/50 backdrop-blur-sm rounded-xl border border-gray-700/50 p-8">
            <h2 className="text-2xl font-semibold mb-8 bg-clip-text text-transparent bg-gradient-to-r from-[#9945FF] to-[#00C2FF]">
              Enterprise-Grade Infrastructure
            </h2>
            <div className="grid md:grid-cols-2 gap-8 mb-8">
              <div className="space-y-4">
                <div className="flex items-center gap-3 text-gray-300">
                  <span className="text-[#14F195]">→</span>
                  Zero block confirmation latency
                </div>
                <div className="flex items-center gap-3 text-gray-300">
                  <span className="text-[#14F195]">→</span>
                  99.99% guaranteed uptime
                </div>
                <div className="flex items-center gap-3 text-gray-300">
                  <span className="text-[#14F195]">→</span>
                  Dedicated bare metal servers
                </div>
                <div className="flex items-center gap-3 text-gray-300">
                  <span className="text-[#14F195]">→</span>
                  No cloud provider dependencies
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                {SERVER_STATS.map((server, i) => (
                  <div key={i} className="bg-gray-900/50 rounded-lg p-4 border border-gray-700/50">
                    <div className="text-xl mb-1">{server.icon}</div>
                    <div className="font-medium text-white">{server.location}</div>
                    <div className="text-sm text-[#14F195]">{server.latency}</div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          <div className="mt-16 grid md:grid-cols-2 gap-8">
            <UseCaseSection
              title="For Traders"
              items={[
                "Set limit orders across chains",
                "Automate DCA strategies",
                "Schedule token conversions",
                "Private transaction routing"
              ]}
            />
            <UseCaseSection
              title="For DeFi Users"
              items={[
                "Manage debt positions",
                "Optimize yield farming",
                "Automate portfolio rebalancing",
                "Time-locked transactions"
              ]}
            />
          </div>
        </main>
      </div>
    </div>
  );
}

function Feature({ title, description }: { title: string; description: string }) {
  return (
    <div className="bg-gray-800/50 backdrop-blur-sm rounded-xl border border-gray-700/50 p-6 hover:scale-105 transition group">
      <h3 className="text-xl font-semibold mb-2 text-transparent bg-clip-text bg-gradient-to-r from-[#9945FF] to-[#00C2FF]">
        {title}
      </h3>
      <p className="text-gray-400">{description}</p>
    </div>
  );
}

function UseCaseSection({ title, items }: { title: string; items: string[] }) {
  return (
    <div className="bg-gray-800/50 backdrop-blur-sm rounded-xl border border-gray-700/50 p-6">
      <h3 className="text-xl font-semibold mb-4 text-transparent bg-clip-text bg-gradient-to-r from-[#9945FF] to-[#00C2FF]">
        {title}
      </h3>
      <ul className="space-y-3">
        {items.map((item, i) => (
          <li key={i} className="flex items-center gap-2 text-gray-300">
            <span className="text-[#14F195]">→</span>
            {item}
          </li>
        ))}
      </ul>
    </div>
  );
}
