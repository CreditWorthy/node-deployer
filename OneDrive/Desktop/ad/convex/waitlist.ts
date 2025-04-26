import { mutation, query } from "./_generated/server";
import { v } from "convex/values";

// Fix: Change to named exports
export const join = mutation({
  args: { email: v.string() },
  handler: async (ctx, args) => {
    // Check if email already exists
    const existing = await ctx.db
      .query("waitlist")
      .withIndex("by_email", q => q.eq("email", args.email))
      .unique();
    
    if (existing) {
      throw new Error("Email already registered");
    }

    await ctx.db.insert("waitlist", {
      email: args.email,
    });
  },
});

export const getCount = query({
  args: {},
  handler: async (ctx) => {
    const count = await ctx.db.query("waitlist").collect();
    return 500 + count.length; // Base 500 + actual signups
  },
});
