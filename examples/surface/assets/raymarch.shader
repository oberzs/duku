vert.spv                                                                                                                    00000005660 00000000000 0005350                                                                                                      ustar                                                                                                                                                                                                                                                          #     8                 GLSL.std.450                      main                 !   %   '   7        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         vertex(      gl_PerVertex             gl_Position         gl_PointSize            gl_ClipDistance         gl_CullDistance               in_position      out_position          out_normal    !   in_normal     %   out_uv    '   in_uv     +   Light     +       coords    +      color     .   WorldObject   .       cam_mat   .      light_mat     .      lights    .      cam_pos   .      time      .      shadow_index      0   world     1   MaterialObject    1       albedo_tint   1      font_width    1      font_border_tint      1      font_edge     1      font_border_offset    1      font_border_width     1      font_border_edge      1      arg_1     1      arg_2     1   	   arg_3     1   
   arg_4     3   material      4   Constants     4       model_mat     4      albedo_index      6   object    7   out_ls_position H                H              H              H              G        G            G            G            G  !         G  %         G  '         H  +       #       H  +      #      G  -          H  .          H  .       #       H  .             H  .         H  .      #   @   H  .            H  .      #   �   H  .      #      H  .      #     H  .      #     G  .      G  0   "       G  0   !       H  1       #       H  1      #      H  1      #      H  1      #      H  1      #       H  1      #   (   H  1      #   ,   H  1      #   0   H  1      #   @   H  1   	   #   P   H  1   
   #   `   G  1      G  3   "      G  3   !       H  4          H  4       #       H  4             H  4      #   @   G  4      G  7              !                   	           
           +  
                         	                        ;                       +                                   ;           +          �?         	               ;           ;            ;     !        #            $      #   ;  $   %         &      #   ;  &   '        *   	        +   	   	   +  
   ,        -   +   ,     .   *   *   -               /      .   ;  /   0        1               #         	   	   	   	      2      1   ;  2   3        4   *         5   	   4   ;  5   6   	   ;     7      6               �     9     )      �  8  6               �     =           Q               Q              Q              P  	                  A              >        =           >        =     "   !   >      "   =  #   (   '   >  %   (   �  8                                                                                  frag.spv                                                                                                                    00000017440 00000000000 0005306                                                                                                      ustar                                                                                                                                                                                                                                                          #                     GLSL.std.450              
       main    �   �                      �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         get_dist_to_scene(vf3;    
   pos      ray_march(vf3;vf3;       ray_orig         ray_dir      get_normal(vf3;      pos      get_light(vf3;       pos      fragment(        sphere    #   sphere_dist   .   plane_dist    2   dist      ;   MAX_STEPS     =   MAX_DIST      ?   SURF_DIST     A   dist_orig     B   i     M   pos   S   dist_scene    T   param     j   dist      k   param     p   e     r   normal    x   param     ~   param     �   param     �   SURF_DIST     �   light_pos     �   Light     �       coords    �      color     �   WorldObject   �       cam_mat   �      light_mat     �      lights    �      cam_pos   �      time      �      shadow_index      �   world     �   light_dir     �   normal    �   param     �   light     �   dist      �   param     �   param     �   uv    �   in_uv     �   ray_orig      �   ray_dir   �   color     �   dist      �   param     �   param     �   pos   �   light     �   param     �   out_color     �   MaterialObject    �       albedo_tint   �      font_width    �      font_border_tint      �      font_edge     �      font_border_offset    �      font_border_width     �      font_border_edge      �      arg_1     �      arg_2     �   	   arg_3     �   
   arg_4       material        Constants           model_mat          albedo_index        object    	  textures        samplers        in_position     in_normal       in_ls_position  H  �       #       H  �      #      G  �          H  �          H  �       #       H  �             H  �         H  �      #   @   H  �            H  �      #   �   H  �      #      H  �      #     H  �      #     G  �      G  �   "       G  �   !       G  �         G  �          H  �       #       H  �      #      H  �      #      H  �      #      H  �      #       H  �      #   (   H  �      #   ,   H  �      #   0   H  �      #   @   H  �   	   #   P   H  �   
   #   `   G  �      G    "      G    !       H           H        #       H              H       #   @   G       G  	  "      G  	  !       G    "      G    !      G           G          G               !                                        !  	         !              !                                  +            +          �?+           �@,     !                   "           )           +  )   *      +  )   /        9             :      9   +  9   <   d   +     >     �B+     @   
�#<+  9   C         K   +  9   e        n            o      n   ,  n   q   @      +     �     �@,     �      �         �           �         +  )   �        �   �   �     �   �   �   �         9      �      �   ;  �   �      +  9   �         �         +     �   ���=   �      n   ;  �   �      +     �      @+     �     ��,     �            +  )   �       ,     �               �         ;  �   �        �               n                              �   ;              �   9        	     ;      	    	                            +  )     d                      ;    	        
      
  *              ;                     ;         ;                    ;         6               �     9     �      �  8  6            	   7     
   �     ;           ;  "   #      ;  "   .      ;  "   2      >     !   =     $   
   =     %      O     &   %   %             �     '   $   &        (      B   '   A  "   +      *   =     ,   +   �     -   (   ,   >  #   -   A  "   0   
   /   =     1   0   >  .   1   =     3   #   =     4   .        5      %   3   4   >  2   5   =     6   2   �  6   8  6               7        7        �     ;  :   ;      ;  "   =      ;  "   ?      ;  "   A      ;  :   B      ;     M      ;  "   S      ;     T      >  ;   <   >  =   >   >  ?   @   >  A      >  B   C   �  D   �  D   �  F   G       �  H   �  H   =  9   I   B   =  9   J   ;   �  K   L   I   J   �  L   E   F   �  E   =     N      =     O      =     P   A   �     Q   O   P   �     R   N   Q   >  M   R   =     U   M   >  T   U   9     V      T   >  S   V   =     W   S   =     X   A   �     Y   X   W   >  A   Y   =     Z   A   =     [   =   �  K   \   Z   [   =     ]   S   =     ^   ?   �  K   _   ]   ^   �  K   `   \   _   �  b       �  `   a   b   �  a   �  F   �  b   �  G   �  G   =  9   d   B   �  9   f   d   e   >  B   f   �  D   �  F   =     g   A   �  g   8  6               7        �     ;  "   j      ;     k      ;  o   p      ;     r      ;     x      ;     ~      ;     �      =     l      >  k   l   9     m      k   >  j   m   >  p   q   =     s   j   =     t      =  n   u   p   O     v   u   u             �     w   t   v   >  x   w   9     y      x   =     z      =  n   {   p   O     |   {   {             �     }   z   |   >  ~   }   9           ~   =     �      =  n   �   p   O     �   �   �             �     �   �   �   >  �   �   9     �      �   P     �   y      �   P     �   s   s   s   �     �   �   �   >  r   �   =     �   r        �      E   �   �  �   8  6            	   7        �     ;  "   �      ;     �      ;     �      ;     �      ;     �      ;  "   �      ;  "   �      ;     �      ;     �      >  �   @   >  �   �   A  �   �   �   �   =     �   �        �         �   A  �   �   �   �   =     �   �        �         �   P  n   �   �   �   =     �   �   O  n   �   �   �          �  n   �   �   �   =     �   �   O     �   �   �            >  �   �   =     �   �   =     �      �     �   �   �        �      E   �   >  �   �   =     �      >  �   �   9     �      �   >  �   �   =     �   �   =     �   �   �     �   �   �        �      +   �         >  �   �   =     �      =     �   �   =     �   �   �     �   �   �   �     �   �   �   >  �   �   =     �   �   >  �   �   9     �      �   �   >  �   �   =     �   �   =     �   �   =     �      �     �   �   �        �      B   �   �  K   �   �   �   �  �       �  �   �   �   �  �   =     �   �   �     �   �   �   >  �   �   �  �   �  �   =     �   �   �  �   8  6               �     ;  o   �      ;     �      ;     �      ;     �      ;  "   �      ;     �      ;     �      ;     �      ;  "   �      ;     �      =  n   �   �   �  n   �   �   �   P  n   �         �  n   �   �   �   >  �   �   A  "   �   �   /   =     �   �   �     �   �   �   A  "   �   �   /   >  �   �   >  �   �   A  "   �   �   �   =     �   �   A  "   �   �   /   =     �   �   P     �   �   �           �      E   �   >  �   �   >  �   �   =     �   �   >  �   �   =     �   �   >  �   �   9     �      �   �   >  �   �   =     �   �   =     �   �   =     �   �   �     �   �   �   �     �   �   �   >  �   �   =     �   �   >  �   �   9     �      �   >  �   �   =     �   �   P     �   �   �   �   >  �   �   =     �   �   Q     �   �       Q     �   �      Q     �   �      P     �   �   �   �      >  �   �   �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  