// Faithful JS mirror of hotel-backend/src/render/thai_id_card.rs::build_svg,
// rendered with the SAME resvg engine (@resvg/resvg-js) + the bundled
// DilleniaUPC-Bold font. Preview only — validates the port before CI builds Rust.
const fs = require('fs');
const { Resvg } = require('@resvg/resvg-js');

const REPO = 'C:/Users/nutok/new-hotel';
const FONT = REPO + '/hotel-backend/assets/fonts/DilleniaUPC-Bold.ttf';
const TEMPLATE = REPO + '/hotel-backend/assets/thai_id_template.png';
const FACE = 'C:/Users/nutok/Downloads/Hotel-2018- V.1.45/109599.jpg';

// --- sample data (same as the GDI+ golden) ---
const F = {
  cid: '1234567890123',
  thai_title: 'นาย', thai_first_name: 'สมชาย', thai_last_name: 'ใจดีมีสุขเกษม',
  english_title: 'Mr.', english_first_name: 'Somchai', english_last_name: 'Jaideemesukkasem',
  date_of_birth: '2533-05-09', issue_date: '2560-03-15', expire_date: '2568-03-14',
  address: '50 หมู่ 5 ตำบลบางแก้ว อำเภอบางพลี จังหวัดสมุทรปราการ',
};

const BLACK = '#000000', NAVY = '#000080';
const TH = ['ม.ค.','ก.พ.','มี.ค.','เม.ย.','พ.ค.','มิ.ย.','ก.ค.','ส.ค.','ก.ย.','ต.ค.','พ.ย.','ธ.ค.'];
const EN = ['Jan.','Feb.','Mar.','Apr.','May.','Jun.','Jul.','Aug.','Sep.','Oct.','Nov.','Dec.'];

const xmlEscape = s => s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;').replace(/'/g,'&apos;');
const pad2 = n => String(n).padStart(2,'0');
function formatCid(raw){const d=(raw.match(/\d/g)||[]).join('');return d.length===13?`${d[0]} ${d.slice(1,5)} ${d.slice(5,10)} ${d.slice(10,12)} ${d.slice(12,13)}`:raw.trim();}
function dateParts(raw){const s=(raw||'').trim();if(!s)return null;const dp=s.split(/[T ]/)[0];const seg=dp.split('-');let y,m,d;if(seg.length===3){y=+seg[0];m=+seg[1];d=+seg[2];}else if(/^\d{8}$/.test(dp)){y=+dp.slice(0,4);m=+dp.slice(4,6);d=+dp.slice(6,8);}else return null;return (m>=1&&m<=12&&d>=1&&d<=31)?[y,m,d]:null;}
function thaiDate(raw){const p=dateParts(raw);if(!p)return'';const[y,m,d]=p;return `${pad2(d)} ${TH[m-1]} ${y>=2400?y:y+543}`;}
function engDate(raw){const p=dateParts(raw);if(!p)return'';const[y,m,d]=p;return `${pad2(d)} ${EN[m-1]} ${y>=2400?y-543:y}`;}
function splitAddress(a){let s=a.trim().split('ตำบล').join('ต.').split('อำเภอ').join('\nอ.').split('จังหวัด').join('จ.').split('แขวง').join('\nแขวง.');const i=s.lastIndexOf('\n');return i<0?[s.trim(),'']:[s.slice(0,i).split('\n').join('').trim(),s.slice(i+1).trim()];}
function joinParts(parts){return parts.map(p=>p.trim()).filter(Boolean).join(' ');}
function offsets(pt,px){if(pt===22)return[25,5];if(pt===18)return[20,4];if(pt===14)return[16,3];return[Math.round(px*0.853),Math.round(px/6)];}
function textEl(x,y,pt,fill,fam,text){if(!text.trim())return'';const px=pt*4/3;const[dy,dx]=offsets(pt,px);return `<text x="${(x+dx).toFixed(2)}" y="${(y+dy).toFixed(2)}" font-family="${fam}" font-weight="bold" font-size="${px.toFixed(3)}" fill="${fill}">${xmlEscape(text)}</text>`;}

const templateB64 = fs.readFileSync(TEMPLATE).toString('base64');
const faceB64 = fs.readFileSync(FACE).toString('base64');
const fam = 'DilleniaUPC';

const cid = formatCid(F.cid);
const thaiName = joinParts([F.thai_title, F.thai_first_name, F.thai_last_name]);
const enFirst = joinParts([F.english_title, F.english_first_name]);
const enLast = F.english_last_name.trim();
const [addr1, addr2] = splitAddress(F.address);

const rows = [
  [200,25,22,BLACK,cid],[110,52,22,BLACK,thaiName],
  [180,78,18,NAVY,enFirst],[205,96,18,NAVY,enLast],
  [190,115,18,BLACK,thaiDate(F.date_of_birth)],[220,135,18,NAVY,engDate(F.date_of_birth)],
  [71,175,18,BLACK,addr1],[44,195,18,BLACK,addr2],
  [44,215,14,BLACK,thaiDate(F.issue_date)],[44,237,14,NAVY,engDate(F.issue_date)],
  [244,215,14,BLACK,thaiDate(F.expire_date)],[244,237,14,NAVY,engDate(F.expire_date)],
];

let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="446" height="273" viewBox="0 0 446 273">`;
svg += `<image href="data:image/png;base64,${templateB64}" x="0" y="0" width="446" height="273" preserveAspectRatio="none"/>`;
for (const [x,y,pt,fill,t] of rows) svg += textEl(x,y,pt,fill,fam,t);
svg += `<image href="data:image/jpeg;base64,${faceB64}" x="332" y="129" width="103" height="119" preserveAspectRatio="xMidYMid slice"/>`;
svg += `</svg>`;

const resvg = new Resvg(svg, {
  font: { fontFiles: [FONT], loadSystemFonts: false, defaultFontFamily: 'DilleniaUPC' },
  fitTo: { mode: 'original' },
});
fs.writeFileSync(REPO + '/.decompile/assets/rust_render_446.png', resvg.render().asPng());
console.log('rendered 446x273 ->', '.decompile/assets/rust_render_446.png');
console.log('addr1:', addr1, '| addr2:', addr2);
