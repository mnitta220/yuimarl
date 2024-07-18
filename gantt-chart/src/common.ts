export const SCROLL_BAR_WIDTH = 16;
export const HEADER_LABEL_Y = 42;
export const LINE_HEIGHT = 21;
export const HEADER_HEIGHT = 63;
export const TICKET_HEIGHT = 22;
export const DAY_WIDTH = 22;
export const CALENDAR_MIN = DAY_WIDTH * 10;
export const DAY_MILISEC = 1000 * 60 * 60 * 24;

export class GanttTicket {
  id: string = "";
  id_disp: string = "";
  name: string = "";
  start_date: string | null = null;
  end_date: string | null = null;
  progress: number = 0;
  open: boolean = false;
  children: GanttTicket[] = [];
}

// カラム
export class Column {
  name: string = "";
  width: number = 0;
}

export interface TicketModalResult {
  result: boolean;
  ticket?: ModalTicket;
  members?: ModalTicketMember[];
  parent?: ModalTicket;
  children?: ModalTicket[];
  message: string;
}

export interface ModalTicket {
  id: string;
  id_disp: string;
  name: string;
  description: string | null;
  start_date: string | null;
  end_date: string | null;
  progress: number;
  priority: number;
}

export interface ModalTicketMember {
  id: string;
  uid: string;
  seq: number;
  name: string;
  email: string;
}
