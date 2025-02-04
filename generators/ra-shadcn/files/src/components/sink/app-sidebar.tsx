"use client"

import * as React from "react"
import { titleCase } from "scule";
import {
  AudioWaveform,
  BookOpen,
  Bot,
  Command,
  Frame,
  GalleryVerticalEnd,
  Map,
  PieChart,
  Settings2,
  SquareTerminal,
} from "lucide-react"

import { NavMain, NavEntry } from "@/components/sink/nav-main"
import { NavProjects } from "@/components/sink/nav-projects"
import { NavUser } from "@/components/sink/nav-user"
import { TeamSwitcher } from "@/components/sink/team-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar"
import { CoreLayoutProps,useResourceDefinitions } from "ra-core";

// This is sample data.
const data = {
  user: {
    name: "shadcn",
    email: "m@example.com",
    avatar: "/avatars/shadcn.jpg",
  },
  teams: [
    {
      name: "Acme Inc",
      logo: GalleryVerticalEnd,
      plan: "Enterprise",
    },
    {
      name: "Acme Corp.",
      logo: AudioWaveform,
      plan: "Startup",
    },
    {
      name: "Evil Corp.",
      logo: Command,
      plan: "Free",
    },
  ],
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const resourceDefinitions = useResourceDefinitions();

  const navEntries:NavEntry[] = Object.values(resourceDefinitions).map(
      (resource: any) => new NavEntry(titleCase(resource.name), `/${resource.name}`, Frame)
  );

  return (
    <Sidebar collapsible="icon" variant="floating" {...props}>
      <SidebarHeader>
        <TeamSwitcher teams={ data.teams} />
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={navEntries} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={data.user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
