import dayjs from "dayjs";

export const SCROLL_BAR_WIDTH = 16;
export const HEADER_LABEL_Y = 42;
export const LINE_HEIGHT = 21;
export const HEADER_HEIGHT = 63;
export const TICKET_HEIGHT = 22;
export const DAY_WIDTH = 22;
export const CALENDAR_MIN = DAY_WIDTH * 10;
export const DAY_MILISEC = 1000 * 60 * 60 * 24;

export class Ticket {
  id: string = "";
  idDisp: string = "";
  name: string = "";
  start: dayjs.Dayjs | null = null;
  end: dayjs.Dayjs | null = null;
  progress: number = 0;
  open: boolean = false;
  children: Ticket[] = [];
}

// カラム
export class Column {
  name: string = "";
  width: number = 0;
}
