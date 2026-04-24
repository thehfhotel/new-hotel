# Receptionist Action Playbook — Legacy DB Capture

We are recording every database query the hotel app sends while you do these
actions. You are NOT testing the app — you are *teaching* us what the app does
behind the scenes by performing a fixed set of normal actions.

## Ground rules

1. **Use ONLY the test inventory** listed below. Do not touch real bookings.
2. **Wait ~10 seconds between each action.** This makes it easier to identify
   in the log which query came from which action.
3. **Use the unique sentinel values** in each step (a marker name, marker phone,
   marker date). Don't substitute. The sentinels let us grep the log later.
4. **Log each action** in the table at the bottom of this file (timestamp +
   what you did + any error). A pen-and-paper note is fine.
5. **If anything fails or asks for confirmation, stop and tell the engineer.**
   Don't dismiss errors — those errors are also data.
6. **Do not close the app between actions.** A fresh login restarts a different
   set of queries we'd have to filter out.

## Test inventory (set up before starting)

| Item                | Value to use throughout                |
|---------------------|----------------------------------------|
| Test room A         | Room **999** (or whatever spare room you have — note here: ____ ) |
| Test room B         | Room **998** (or note here: ____ ) |
| Test customer name  | `SPIKE TEST 1` (use exactly this, all caps) |
| Test customer phone | `0900000001` |
| Test customer ID #  | `1100000000001` (Thai ID format, dummy) |
| Test booking dates  | Arrival = today, Departure = today + 2 days |
| Test second guest   | `SPIKE TEST 2` / `0900000002` / `1100000000002` |
| Test rate / price   | Whatever the app suggests — don't override |

## Actions (in order)

Do each action once, log it, wait 10 seconds, move on.

### Group 1 — Customer & booking creation
1. **Create new customer** named `SPIKE TEST 1` with phone `0900000001` and ID `1100000000001`.
   Save. Log the timestamp.
2. **Search for that customer.** Open their detail screen. Close it.
3. **Create new booking** for `SPIKE TEST 1`, room A (999), arrival today, departure today+2.
   Save. Note the booking number the system assigned: ____
4. **Open the booking** you just created. Just open & view, no changes.
5. **Add a note / remark** to the booking (free text: "spike test note 1"). Save.
6. **Modify the booking** — change departure to today+3. Save.

### Group 2 — Check-in
7. **Walk-in (no prior booking)** for room B (998) with a NEW customer
   `SPIKE TEST 2` (phone `0900000002`, ID `1100000000002`). Use today as arrival
   and today+1 as departure. Save & check in.
8. **Check in the booking from step 3** (room A, SPIKE TEST 1). If the app asks
   for ID details, use the test values.
9. **Add a second guest** to the room A check-in (associate `SPIKE TEST 2` as
   accompanying guest, if your app supports this — TM.30 registration). Save.

### Group 3 — Charges & payments
10. **Post a charge** to room A (e.g. minibar / laundry / breakfast — whatever
    your app's smallest charge is, ~50-100 baht). Save.
11. **Take a partial payment** on room A's folio — pay ~500 baht cash. Save.
12. **Print the invoice / receipt** for room A. Even if the printer is offline,
    the print preview triggers the same queries.

### Group 4 — Modifications & exits
13. **Move room A guest** to a different room (if your app supports room-change).
    Skip if not applicable.
14. **Cancel the room B walk-in's check-in** (then either delete the booking or
    cancel it — whichever is the normal path).
15. **Check out room A.** Settle the balance to zero with cash. Final receipt.
16. **Mark room A as "dirty" / "needs cleaning"** (housekeeping status), if the
    app surfaces this.
17. **Mark room A as "clean / ready"** after.

### Group 5 — Reads (no writes — but useful to capture the read patterns)
18. Open the **today's arrivals** report (or whatever screen shows today's check-ins).
19. Open the **today's departures** report.
20. Open the **room availability calendar** for the next 7 days.
21. Open the **occupancy / revenue report** for today.
22. Open the **guest registry / TM.30 export** (if your app has one).

## Done

Tell the engineer "done." They will run `finalize.sh` to stop the capture.

## Action log (fill in as you go)

| #  | Time (HH:MM) | Result | Notes / errors |
|----|--------------|--------|----------------|
| 1  |              |        |                |
| 2  |              |        |                |
| 3  |              | booking# = ____ |          |
| 4  |              |        |                |
| 5  |              |        |                |
| 6  |              |        |                |
| 7  |              |        |                |
| 8  |              |        |                |
| 9  |              |        |                |
| 10 |              |        |                |
| 11 |              |        |                |
| 12 |              |        |                |
| 13 |              |        |                |
| 14 |              |        |                |
| 15 |              |        |                |
| 16 |              |        |                |
| 17 |              |        |                |
| 18 |              |        |                |
| 19 |              |        |                |
| 20 |              |        |                |
| 21 |              |        |                |
| 22 |              |        |                |

## Cleanup (after the engineer has the data)

The test customers `SPIKE TEST 1` and `SPIKE TEST 2` and any test bookings
should be deleted from the system once the engineer confirms the data is
captured. Use the app's normal customer/booking deletion flow.
